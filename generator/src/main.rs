use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use schemars::schema::{InstanceType, Metadata, Schema, SingleOrVec};
use schemars::visit::visit_root_schema;
use schemars::{
    schema::{RootSchema, SchemaObject},
    visit::{visit_schema_object, Visitor},
};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_dir;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::{fs, fs::File, io::BufWriter};
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to open schema {path}: {inner}")]
    FailedToOpenSchema {
        path: PathBuf,
        inner: Box<dyn std::error::Error>,
    },
    #[error("Unhandled instance type")]
    UnhandledInstanceType(Option<SingleOrVec<InstanceType>>),
    #[error("Unhandled array item type")]
    UnhandledArrayItemType,
}

struct Enum {
    options: Vec<String>,
}

struct ArrayType {
    min_length: Option<u32>,
    item: Box<Type>,
}

struct FixedArrayType {
    item: Box<Type>,
    length: u32,
}

enum Type {
    Any,
    Array(ArrayType),
    FixedArray(FixedArrayType),
    TypedObject(String),
    UntypedObject,
    String,
    Boolean,
    Number,
    Integer,
    Enum(Enum),
    MapOfObjects,
}

#[derive(Clone)]
struct SchemaContext<'a> {
    schema_store: &'a SchemaStore<'a>,
    schema: &'a SchemaObject,
}

impl<'a> SchemaContext<'a> {
    fn resolve<'b, 'c>(&'b self, schema: &'b SchemaObject) -> SchemaContext<'c>
    where
        'b: 'c,
    {
        if schema.is_ref() {
            return SchemaContext {
                schema_store: self.schema_store,
                schema: self
                    .schema_store
                    .lookup(schema.reference.as_ref().unwrap())
                    .unwrap(),
            };
        }

        SchemaContext {
            schema_store: self.schema_store,
            schema,
        }
    }
}

fn handle_field(schema: &SchemaContext) -> Result<Type, Box<dyn Error>> {
    // If we have an allOf with a single entry we can use it as our type
    if let Some(subschema) = &schema.schema.subschemas {
        if let Some(Schema::Object(single_all_of)) =
            subschema.all_of.as_ref().and_then(|all_of| all_of.first())
        {
            let the_schema = schema.resolve(single_all_of);
            if let Some(_) = the_schema.schema.object {
                if let Some(id) = the_schema
                    .schema
                    .metadata
                    .as_ref()
                    .and_then(|md| md.id.as_ref())
                {
                    return Ok(Type::TypedObject(id.clone()));
                }
            }
        }
    }

    if let Some(enumeration) = try_match_string_enum(schema) {
        return Ok(Type::Enum(enumeration));
    }

    if try_match_int_enum(schema).is_some() {
        return Ok(Type::Integer);
    }

    handle_type(schema)
}

fn try_match_string_enum(schema: &SchemaContext) -> Option<Enum> {
    let any_of = match schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|subschema| subschema.any_of.as_ref())
    {
        Some(any_of) => any_of,
        _ => return None,
    };

    let mut options = Vec::new();
    for option in any_of {
        let option = match option {
            Schema::Object(object) => object,
            _ => return None,
        };

        let is_string_constant = match option.const_value.as_ref() {
            Some(Value::String(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_string = match option.instance_type.as_ref() {
            Some(SingleOrVec::Single(single)) => match single.as_ref() {
                InstanceType::String => true,
                _ => false,
            },
            _ => false,
        };

        if !is_string && !is_string_constant {
            return None;
        }
    }

    Some(Enum { options })
}

fn try_match_int_enum(schema: &SchemaContext) -> Option<()> {
    let any_of = match schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|subschema| subschema.any_of.as_ref())
    {
        Some(any_of) => any_of,
        _ => return None,
    };

    let mut options = Vec::new();
    for option in any_of {
        let option = match option {
            Schema::Object(object) => object,
            _ => return None,
        };

        let is_number_constant = match option.const_value.as_ref() {
            Some(Value::Number(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_number = match option.instance_type.as_ref() {
            Some(SingleOrVec::Single(single)) => match single.as_ref() {
                InstanceType::Integer => true,
                _ => false,
            },
            _ => false,
        };

        if !is_number && !is_number_constant {
            return None;
        }
    }

    Some(())
}

fn handle_type(schema: &SchemaContext) -> Result<Type, Box<dyn Error>> {
    // Try to match based on an instance type if one exists
    let ty = match &schema.schema.instance_type {
        Some(SingleOrVec::Single(a)) => match **a {
            InstanceType::Null => Err(Box::new(MyError::UnhandledInstanceType(
                schema.schema.instance_type.clone(),
            )) as Box<dyn Error>),
            InstanceType::Boolean => Ok(Some(Type::Boolean)),
            InstanceType::Object => {
                // An object with no properties, but only additionalProperties, as a typed map
                if let Some(_) = schema
                    .schema
                    .object
                    .as_ref()
                    .unwrap()
                    .additional_properties
                    .as_ref()
                {
                    if schema.schema.object.as_ref().unwrap().properties.is_empty() {
                        Ok(Some(Type::MapOfObjects))
                    } else {
                        unreachable!();
                    }

                // If the object has a title, it's a typed object
                } else if let Some(id) = schema
                    .schema
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.id.as_ref())
                {
                    Ok(Some(Type::TypedObject(id.clone())))
                } else {
                    Ok(Some(Type::UntypedObject))
                }
            }
            InstanceType::Array => Ok(Some(handle_array(schema)?)),
            InstanceType::Number => Ok(Some(Type::Number)),
            InstanceType::String => Ok(Some(Type::String)),
            InstanceType::Integer => Ok(Some(Type::Integer)),
        },
        _ => Ok(None),
    }?;

    match ty {
        Some(ty) => return Ok(ty),
        _ => (),
    };

    // If there is an allOf with a single entry try to match based of this instead
    if let Some(Schema::Object(single_all_of)) = schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|schema| schema.all_of.as_ref())
        .and_then(|all_of| all_of.first())
    {
        let single_all_of = schema.resolve(single_all_of);
        return handle_type(&single_all_of);
    }

    Ok(Type::Any)
}

fn handle_array(schema: &SchemaContext) -> Result<Type, Box<dyn Error>> {
    let array = schema.schema.array.as_ref().unwrap();

    let single_item_type = match array.items.as_ref() {
        Some(SingleOrVec::Single(ty)) => match ty.as_ref() {
            Schema::Object(object) => object,
            _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
        },
        _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
    };

    let single_item_type = schema.resolve(single_item_type);

    let item_type = handle_type(&single_item_type)?;

    if array.min_items.is_some() && array.min_items == array.max_items {
        let fixed_length = array.min_items.unwrap();
        return Ok(Type::FixedArray(FixedArrayType {
            item: Box::new(item_type),
            length: fixed_length,
        }));
    }

    Ok(Type::Array(ArrayType {
        min_length: array.min_items,
        item: Box::new(item_type),
    }))
}

fn generate_rust_type(
    schema_lookup: &HashMap<String, SchemaContext>,
    ty: &Type,
    field_name: &String,
) -> TokenStream {
    match ty {
        Type::Any => quote! { serde_json::Value },
        Type::Array(array_type) => {
            let item_rust_type = generate_rust_type(schema_lookup, &array_type.item, field_name);
            return quote! { Vec::< #item_rust_type > };
        }
        Type::FixedArray(array_type) => {
            let fixed_length = array_type.length as usize;
            let rust_item_type = generate_rust_type(schema_lookup, &array_type.item, field_name);
            quote! { [#rust_item_type; #fixed_length ]}
        }
        Type::Boolean => quote! { bool },
        Type::Integer => quote! { i64 },
        Type::Number => quote! { f64 },
        Type::String => quote! { String },
        Type::Enum(_) => {
            let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site());
            quote! { #ident }
        }
        Type::TypedObject(id) => {
            let metadata = schema_lookup
                .get(id)
                .unwrap()
                .schema
                .metadata
                .as_ref()
                .unwrap();

            let module = generate_module_identifier(metadata);
            let ident = generate_struct_identifier(metadata);
            quote! { super::#module::#ident }
        }
        Type::UntypedObject => quote! { Map<String, Value> },
        Type::MapOfObjects => quote! { Map<String, Value> },
    }
}

fn schedule_types(open_types: &mut Vec<String>, closed_types: &HashSet<String>, ty: &Type) {
    match ty {
        Type::Array(array_type) => {
            schedule_types(open_types, closed_types, array_type.item.as_ref())
        }
        Type::TypedObject(id) => {
            if !closed_types.contains(id) && !open_types.contains(id) {
                open_types.push(id.clone());
            }
        }
        _ => (),
    }
}

struct Property {
    ty: Type,
    optional: bool,
    default: Option<Value>,
    comment: Option<String>,
}

fn recursive_read_properties(properties: &mut HashMap<String, Property>, schema: &SchemaContext) {
    // First read properties from 'base' schemas
    let base_schema = schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|subschema| subschema.all_of.as_ref())
        .and_then(|all_of| all_of.first());
    if let Some(Schema::Object(base)) = base_schema {
        let base = schema.resolve(base);
        recursive_read_properties(properties, &base);
    }

    // Then add our own properties
    let object_schema = schema.schema.object.as_ref().unwrap();
    for (name, field_schema) in object_schema.properties.iter() {
        let field_schema = match field_schema {
            Schema::Object(object) => object,
            _ => unreachable!(),
        };
        let field_schema = schema.resolve(field_schema);

        let property = properties.entry(name.clone()).or_insert(Property {
            ty: Type::Any,
            optional: true,
            comment: None,
            default: None,
        });

        match property.ty {
            Type::Any => property.ty = handle_field(&field_schema).unwrap(),
            _ => (),
        }

        if property.comment.is_none() {
            property.comment = field_schema
                .schema
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.description.clone());
        }

        if property.default.is_none() {
            property.default = field_schema
                .schema
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.default.clone());
        }

        if object_schema.required.contains(name) {
            property.optional = false;
        }
    }
}

fn generate_default_value_token(ty: &Type, default: &Value, field_name: &String) -> TokenStream {
    match ty {
        Type::Any => unimplemented!(),
        Type::Array(_) => quote! {{ Vec::default(); }},
        Type::FixedArray(array) => {
            let array_items = default
                .as_array()
                .unwrap()
                .iter()
                .map(|item| generate_default_value_token(&array.item, item, field_name));
            quote! { [ #(#array_items),* ]}
        }
        Type::Boolean => {
            let value = default.as_bool().unwrap();
            quote! { #value }
        }
        Type::Enum(_) => {
            let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site());
            quote! { #ident::default() }
        }
        Type::Integer => {
            let integer: i64 = default.as_i64().unwrap();
            quote! { #integer }
        }
        Type::MapOfObjects => quote! { Map<String, Value>::default() },
        Type::Number => {
            let number: f64 = default.as_f64().unwrap();
            quote! { #number }
        }
        Type::String => unimplemented!(),
        Type::TypedObject(_) => unimplemented!(),
        Type::UntypedObject => unimplemented!(),
    }
}

fn generate_module_identifier(metadata: &Box<Metadata>) -> Ident {
    let title = metadata.as_ref().title.as_ref().unwrap().to_lowercase();
    Ident::new(&title.to_case(Case::Snake), Span::call_site())
}
fn generate_struct_identifier(metadata: &Box<Metadata>) -> Ident {
    let title = metadata.as_ref().title.as_ref().unwrap().to_lowercase();
    Ident::new(&title.to_case(Case::UpperCamel), Span::call_site())
}

fn generate_property_identifier(name: &str) -> Ident {
    Ident::new(
        &name.to_case(Case::Snake).replace("type", "ty"),
        Span::call_site(),
    )
}

/// Writes a rust type into a unique module with helper functions and type surrounding it
struct RustTypeWriter<'a> {
    open_types: &'a mut Vec<String>,
    closed_types: &'a HashSet<String>,
    embedded_enums: Vec<TokenStream>,
    default_declarations: Vec<TokenStream>,
}

impl<'a> RustTypeWriter<'a> {
    fn new<'b>(open_types: &'b mut Vec<String>, closed_types: &'b HashSet<String>) -> Self
    where
        'b: 'a,
    {
        Self {
            open_types,
            closed_types,
            embedded_enums: Vec::new(),
            default_declarations: Vec::new(),
        }
    }
}

fn write_property(
    writer: &mut RustTypeWriter,
    name: &String,
    property: &Property,
    schema_lookup: &HashMap<String, SchemaContext>,
) -> TokenStream {
    // Ensure that the type referenced by our property will also be written out
    schedule_types(writer.open_types, writer.closed_types, &property.ty);

    let rust_type = match (&property.ty, property.optional) {
        // Remove the Option for optional Vec's with a minimum length of 1
        // This way we can guarantee this invariant by telling serde to not serialize zero length vecs.
        (Type::Array(array_type), true)
            if array_type.min_length.is_some() && array_type.min_length.unwrap() == 1 =>
        {
            generate_rust_type(schema_lookup, &property.ty, name)
        }

        // Remove the Option for optional properties which have a default value specified.
        (_, true) if property.default.is_none() => {
            let rust_type: TokenStream = generate_rust_type(schema_lookup, &property.ty, name);
            quote! { Option::<#rust_type> }
        }
        _ => generate_rust_type(schema_lookup, &property.ty, name),
    };

    // Embedded enums
    if let Type::Enum(enumeration) = &property.ty {
        let rusty_enum_name = Ident::new(&name.to_case(Case::UpperCamel), Span::call_site());
        let enum_options = enumeration.options.iter().map(|option| {
            let identifier = Ident::new(
                &option.to_case(Case::UpperCamel).replace(&['/'], ""),
                Span::call_site(),
            );

            let is_default = match &property.default {
                Some(Value::String(string)) => string == option,
                _ => false,
            };

            let default_declaration = is_default.then(|| quote! { #[default] });
            quote! {
                #[serde(rename=#option)]
                #default_declaration
                #identifier
            }
        });

        let default_declaration = property
            .default
            .as_ref()
            .map(|_| quote! { #[derive(Default)] });
        writer.embedded_enums.push(quote! {
            #[derive(Serialize, Deserialize, Debug)]
            #default_declaration
            enum #rusty_enum_name{
                #(#enum_options),*
            }
        });
    }

    // For objects with an explicit default, create a default declaration
    let explicit_default_value = property
        .default
        .as_ref()
        .map(|default| generate_default_value_token(&property.ty, default, name));
    let default_declaration = explicit_default_value.as_ref().map(|_| {
        Ident::new(
            &format!("get_default_{}", name.to_case(Case::Snake)),
            Span::call_site(),
        )
    });

    if let Some(default_declaration) = &default_declaration {
        writer.default_declarations.push(quote! {
            fn #default_declaration() -> #rust_type{
                #explicit_default_value
            }
        });
    }

    let default_declaration = default_declaration
        .as_ref()
        .map(|declaration| {
            let string = declaration.to_string();
            quote! { #[serde(default=#string)]}
        })
        .or_else(|| match property.optional {
            true => Some(quote! { #[serde(default)] }),
            false => None,
        });

    // If the property identifier is different from the one in the spec we need to add a serde
    // rename to make it match the spec.
    let property_ident = generate_property_identifier(name);
    let rename_declaration =
        if property_ident.to_string().partial_cmp(&name) != Some(Ordering::Equal) {
            Some(quote![#[serde(rename = #name)]])
        } else {
            None
        };

    let docstring = property.comment.as_ref().map(|x| quote! { #[doc=#x] });
    quote! {
        #rename_declaration
        #default_declaration
        #docstring
        #property_ident: #rust_type
    }
}

fn generate_structure(
    mod_identifier: &Ident,
    open_types: &mut Vec<String>,
    closed_types: &HashSet<String>,
    schema_lookup: &HashMap<String, SchemaContext>,
    name: &Ident,
    comment: Option<&String>,
    schema: &SchemaContext,
) -> TokenStream {
    let mut properties = HashMap::new();
    recursive_read_properties(&mut properties, &schema);
    let mut property_tokens = Vec::new();
    let mut type_writer = RustTypeWriter::new(open_types, closed_types);
    for (name, property) in properties.iter() {
        property_tokens.push(write_property(
            &mut type_writer,
            name,
            property,
            schema_lookup,
        ));
    }
    let doc = match comment {
        Some(comment) => Some(quote! { #[doc=#comment]}),
        _ => None,
    };

    let embedded_enums = &type_writer.embedded_enums;
    let default_declarations = &type_writer.default_declarations;

    quote! {
        pub mod #mod_identifier{
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};

            #(#embedded_enums)*

            #[derive(Serialize, Deserialize, Debug)]
            #doc
            pub struct #name{
                #(#property_tokens),*
            }

            #(#default_declarations)*

        }
    }
}

fn write_rust(
    schema_lookup: &HashMap<String, SchemaContext>,
    schema: &SchemaContext,
    writer: &mut dyn std::io::Write,
    open_types: &mut Vec<String>,
    closed_types: &HashSet<String>,
) {
    let metadata = schema.schema.metadata.as_ref().unwrap();

    let mod_identifier = generate_module_identifier(metadata);
    let comment = metadata.description.as_ref();
    let name = generate_struct_identifier(metadata);

    let tokens = generate_structure(
        &mod_identifier,
        open_types,
        closed_types,
        schema_lookup,
        &name,
        comment,
        schema,
    );
    let file: syn::File = syn::parse2(tokens).unwrap();
    write!(writer, "{}", prettyplease::unparse(&file)).unwrap();
}

struct ReferencedSchemaVisitor<'a, 'b> {
    store: &'b mut SchemaStore<'a>,
    result: Result<(), Box<dyn Error>>,
}

impl<'a, 'b> Visitor for ReferencedSchemaVisitor<'a, 'b> {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        // Don't load more references once one fails
        if self.result.is_err() {
            return;
        }

        if !schema.is_ref() {
            visit_schema_object(self, schema);
            return;
        }

        // Try to load it from the base store first
        if let Some(base_store) = self.store.base {
            if base_store
                .schemas
                .contains_key(schema.reference.as_ref().unwrap())
            {
                return;
            }
        }

        // Resolve the schema by path
        self.result = self.store.read(schema.reference.as_ref().unwrap());
    }
}

struct SchemaStore<'a> {
    folder: PathBuf,
    schemas: HashMap<String, RootSchema>,
    roots: Vec<String>,
    base: Option<&'a SchemaStore<'a>>,
}

impl<'a> SchemaStore<'a> {
    fn new_root(folder: &Path) -> Self {
        Self {
            folder: PathBuf::from(folder),
            base: None,
            roots: Vec::new(),
            schemas: HashMap::new(),
        }
    }

    #[allow(unused)]
    fn new_extension(base: &'a SchemaStore<'a>, folder: &Path) -> Self {
        Self {
            folder: PathBuf::from(folder),
            base: Some(base),
            roots: Vec::new(),
            schemas: HashMap::new(),
        }
    }

    fn read_root(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        let result = self.read(id);
        self.roots.push(id.into());
        result
    }

    fn read(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        let mut full_path = self.folder.clone();
        full_path.push(id);

        // Read the requested schema
        let file = File::open(&full_path).map_err(|e| MyError::FailedToOpenSchema {
            path: full_path.clone(),
            inner: Box::new(e),
        })?;
        let reader = BufReader::new(file);
        let mut root_schema =
            serde_json::from_reader(reader).map_err(|e| MyError::FailedToOpenSchema {
                path: full_path.clone(),
                inner: Box::new(e),
            })?;

        // Read any requested subschema
        let mut visitor = ReferencedSchemaVisitor {
            store: self,
            result: Result::Ok(()),
        };
        visit_root_schema(&mut visitor, &mut root_schema);
        self.schemas.insert(id.to_string(), root_schema);
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&SchemaObject> {
        // Try in base first
        if let Some(base) = self.base {
            match base.lookup(name) {
                Some(object) => return Some(object),
                _ => (),
            }
        }

        // Try ourselves
        self.schemas.get(name).map(|schema| &schema.schema)
    }
}

#[allow(unused)]
fn load_extensions(
    generated_manifest: &mut GeneratedManifest,
    extensions_path: &str,
    generated_path: &str,
    specification_schema: &SchemaStore,
) -> Result<(), String> {
    for entry in read_dir(extensions_path)
        .expect("Failed to open extensions directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_type()
                .map_or(false, |file_type| file_type.is_dir())
        })
    {
        // Figure out the extension name and vendor prefix
        let extension_name = entry.file_name().to_string_lossy().to_string();
        let _= extension_name
            .split('_')
            .next()
            .expect("Extension does not start with vendor prefix followed by an underscore");

        let extension_module_name = extension_name.to_case(Case::Snake);

        // Now we find extension schemas in the schema subfolder
        // Note not all extensions actually provide a schema if they don't add an extension object
        let mut schemas_path = entry.path();
        schemas_path.push("schema");
        let extension_schema_suffix = format!("{}.schema.json", &extension_name);

        let mut extension_schema_store =
            SchemaStore::new_extension(specification_schema, &schemas_path);

        let schemas_dir = match read_dir(schemas_path) {
            Ok(schemas) => schemas,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("Extension {} does not provide any schemas", extension_name);
                continue;
            }
            Err(_) => return Err(String::from("Failed to open schemas directory")),
        };

        let mut extension_module = Vec::new();

        for schema_entry in schemas_dir.filter_map(Result::ok).filter(|entry| {
            entry
                .file_type()
                .map_or(false, |file_type| file_type.is_file())
        }) {
            // If a schema ends with {Prefix}.ExtensionName.schema.json it represents the extension object with the extension name on that object
            let file_name = schema_entry.file_name().to_string_lossy().to_string();
            let suffix_start = match file_name.find(&extension_schema_suffix) {
                Some(index) if index == file_name.len() - extension_schema_suffix.len() => index,
                _ => continue,
            };

            let base_object_name = &file_name[0..suffix_start];
            // TODO: Empty base object name seems to mean it applies to all
            if base_object_name.is_empty() {
                continue;
            }

            println!(
                "Extension {} has an extension on {}",
                &extension_name, &base_object_name
            );

            let base_object_module_ident = Ident::new(
                &base_object_name.replace(".", " ").to_case(Case::Snake),
                Span::call_site(),
            );
            let extension_doc = Some(format!(
                "The {extension_name} extension for {base_object_name}"
            ));

            extension_schema_store.read_root(&file_name).unwrap();

            let lookup = build_schena_lookup(&extension_schema_store);
            let schema = lookup.get(&file_name).unwrap();

            let mut open_types = Vec::new();
            let closed_types = HashSet::new();

            extension_module.push(generate_structure(
                &base_object_module_ident,
                &mut open_types,
                &closed_types,
                &lookup,
                &Ident::new("Extension", Span::call_site()),
                extension_doc.as_ref(),
                schema,
            ));
        }

        let output = File::create(format!("{generated_path}\\{extension_module_name}.rs")).unwrap();
        let mut writer = BufWriter::new(output);

        let rust_file: syn::File = syn::parse2(quote! {
            #(#extension_module)*
        })
        .unwrap();

        write!(writer, "{}", prettyplease::unparse(&rust_file)).unwrap();

        generated_manifest
            .extension_modules
            .push(extension_module_name);
    }

    Ok(())
}

fn ensure_empty_dir(path: &str) {
    match read_dir(path) {
        Ok(dir) => {
            // Directory was found, remove any entries if they exist
            for entry in dir {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        fs::remove_dir_all(path).expect("Failed to remove a dir");
                    } else {
                        fs::remove_file(path).expect("Failed to remove a file");
                    }
                };
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Directory was not found, create it
            fs::create_dir(path).unwrap();
        }
        Err(e) => panic!("Unhandled error {e}"),
    }
}

struct GeneratedManifest {
    extension_modules: Vec<String>,
}

impl GeneratedManifest {
    fn new() -> Self {
        Self {
            extension_modules: Vec::new(),
        }
    }
}

fn write_root_module(generated_path: &str, generated_manifest: &GeneratedManifest) {
    let output = File::create(format!("{generated_path}\\mod.rs")).unwrap();
    let mut writer = BufWriter::new(output);

    let extension_modules: Vec<TokenStream> = generated_manifest
        .extension_modules
        .iter()
        .map(|module_name| {
            let ident = Ident::new(module_name, Span::call_site());
            quote! { pub mod #ident; }
        })
        .collect();

    let rust_file: syn::File = syn::parse2(quote! {
        pub mod gltf;
        #(#extension_modules)*
    })
    .unwrap();

    write!(writer, "{}", prettyplease::unparse(&rust_file)).unwrap();
}

fn create_specification_schema_store() -> SchemaStore<'static> {
    let mut specification_schema =
        SchemaStore::new_root(&PathBuf::from("vendor\\gltf\\specification\\2.0\\schema"));
    specification_schema.read_root("glTF.schema.json").unwrap();
    specification_schema
}

fn add_schema_store_lookup<'a, 'b>(
    schema_store: &'a SchemaStore,
    lookup: &mut HashMap<String, SchemaContext<'b>>,
) where
    'a: 'b,
{
    // Base schemas
    if let Some(base) = schema_store.base {
        add_schema_store_lookup(base, lookup);
    }

    // My schemas
    for (path, schema) in schema_store.schemas.iter() {
        lookup.insert(
            path.clone(),
            SchemaContext {
                schema_store: &schema_store,
                schema: &schema.schema,
            },
        );
    }
}

fn build_schena_lookup<'a, 'b>(schema_store: &'b SchemaStore) -> HashMap<String, SchemaContext<'a>>
where
    'b: 'a,
{
    let mut schema_lookup = HashMap::new();
    add_schema_store_lookup(schema_store, &mut schema_lookup);
    schema_lookup
}

fn main() {
    // Recreate the generated directory
    let generated_path = "gltf_for_rust\\src\\generated";
    ensure_empty_dir(generated_path);

    // Create the core specification schema store
    let specification_schema_store = create_specification_schema_store();

    let generated_manifest = GeneratedManifest::new();
    //load_extensions(&mut generated_manifest, "vendor\\gltf\\extensions\\2.0\\Khronos", generated_path, &specification_schema_store).unwrap();

    let output = File::create(format!("{generated_path}\\gltf.rs")).unwrap();
    let mut writer = BufWriter::new(output);

    let schema_lookup = build_schena_lookup(&specification_schema_store);

    // Collect root types:
    let mut closed_types = HashSet::new();
    let mut open_types = Vec::new();
    for path in specification_schema_store.roots.iter() {
        let schema = specification_schema_store.schemas.get(path).unwrap();
        if let Some(id) = schema
            .schema
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.id.as_ref())
        {
            open_types.push(id.clone());
        }
    }

    while !open_types.is_empty() {
        let id = open_types.pop().unwrap();
        closed_types.insert(id.clone());
        let schema = *schema_lookup.get(&id).as_ref().unwrap();

        write_rust(
            &schema_lookup,
            schema,
            &mut writer,
            &mut open_types,
            &closed_types,
        );
    }

    write_root_module(generated_path, &generated_manifest);
}
