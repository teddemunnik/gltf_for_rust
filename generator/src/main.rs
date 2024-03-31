mod schema;

use crate::schema::{SchemaType, SchemaUri};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use schema::{SchemaContext, SchemaStore};
use schemars::schema::{InstanceType, Metadata, Schema, SingleOrVec};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{OpenOptions, read_dir};
use std::io::Write;
use std::path::PathBuf;
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
    TypedObject(SchemaUri),
    EmbeddedObject(ObjectPrototype),
    String,
    Boolean,
    Number,
    Integer,
    Enum(Enum),
    MapOfObjects,
}

struct ObjectPrototype{
    comment: Option<String>,
    properties: HashMap<String, Property>
}

struct ObjectType {
    name: String,
    prototype: ObjectPrototype,
}

fn handle_field(schema: &SchemaContext, open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>) -> Result<Type, Box<dyn Error>> {
    if let Some(enumeration) = try_match_string_enum(schema) {
        return Ok(Type::Enum(enumeration));
    }

    if try_match_int_enum(schema).is_some() {
        return Ok(Type::Integer);
    }

    handle_type(schema, open_types, closed_types)
}

fn try_match_string_enum(schema: &SchemaContext) -> Option<Enum> {
    let any_of = match schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|sub_schema| sub_schema.any_of.as_ref())
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
        .and_then(|sub_schema| sub_schema.any_of.as_ref())
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

fn handle_object_type(schema: &SchemaContext, open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>) -> Result<Option<Type>, Box<dyn Error>> {
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
            return Ok(Some(Type::MapOfObjects));
        }
    }

    if schema.is_uri_root{
        let uri = schema.uri.as_ref().unwrap();
        return Ok(Some(Type::TypedObject(uri.clone())));
    }

    // Embedded object
    let comment = schema.schema.metadata.as_ref().and_then(|metadata| metadata.description.clone());
    let mut properties = HashMap::new();
    recursive_read_properties(&mut properties, &schema, open_types, closed_types);
    Ok(Some(Type::EmbeddedObject(ObjectPrototype{
        properties,
        comment
    })))
}
fn handle_type_from_instance_type(schema: &SchemaContext, open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>) -> Result<Option<Type>, Box<dyn Error>> {
    // Try to match based on an instance type if one exists
    match &schema.schema.instance_type {
        Some(SingleOrVec::Single(a)) => match **a {
            InstanceType::Null => Err(Box::new(MyError::UnhandledInstanceType(
                schema.schema.instance_type.clone(),
            )) as Box<dyn Error>),
            InstanceType::Boolean => Ok(Some(Type::Boolean)),
            InstanceType::Object => handle_object_type(schema, open_types, closed_types),
            InstanceType::Array => Ok(Some(handle_array(schema, open_types, closed_types)?)),
            InstanceType::Number => Ok(Some(Type::Number)),
            InstanceType::String => Ok(Some(Type::String)),
            InstanceType::Integer => Ok(Some(Type::Integer)),
        },
        _ => Ok(None),
    }
}

fn handle_type(schema: &SchemaContext, open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>) -> Result<Type, Box<dyn Error>> {
    if let Some(ty) = handle_type_from_instance_type(schema, open_types, closed_types)? {
        return Ok(ty);
    }
    // If there is an allOf with a single entry try to match based of this instead
    if let Some(Schema::Object(single_all_of)) = schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|schema| schema.all_of.as_ref())
        .and_then(|all_of| all_of.first())
    {
        let single_all_of = schema.resolve(single_all_of);
        return handle_type(&single_all_of, open_types, closed_types);
    }

    // Fallback to an any
    Ok(Type::Any)
}

fn handle_array(schema: &SchemaContext, open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>) -> Result<Type, Box<dyn Error>> {
    let array = schema.schema.array.as_ref().unwrap();

    let single_item_type = match array.items.as_ref() {
        Some(SingleOrVec::Single(ty)) => match ty.as_ref() {
            Schema::Object(object) => object,
            _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
        },
        _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
    };

    let single_item_type = schema.resolve(single_item_type);

    let item_type = handle_type(&single_item_type, open_types, closed_types)?;

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

fn generate_named_type_path(store: &SchemaStore, uri: &SchemaUri) -> TokenStream {
    let ty = &store.lookup(uri).unwrap().0.ty;
    let name = get_raw_name(store, uri);
    let type_name = Ident::new(&name.to_case(Case::UpperCamel), Span::call_site());

    match ty {
        SchemaType::Specification => quote! { crate::generated::gltf::#type_name},
        SchemaType::Extension(name) => {
            let extension_module = Ident::new(&name.to_lowercase(), Span::call_site());
            quote! { crate::generated::#extension_module::#type_name }
        }
    }
}

fn generate_rust_type(schema_store: &SchemaStore, ty: &Type, field_name: &String) -> TokenStream {
    match ty {
        Type::Any => quote! { serde_json::Value },
        Type::Array(array_type) => {
            let item_rust_type = generate_rust_type(schema_store, &array_type.item, field_name);
            return quote! { Vec::< #item_rust_type > };
        }
        Type::FixedArray(array_type) => {
            let fixed_length = array_type.length as usize;
            let rust_item_type = generate_rust_type(schema_store, &array_type.item, field_name);
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
        Type::TypedObject(uri) => generate_named_type_path(schema_store, uri),
        Type::MapOfObjects => quote! { Map<String, Value> },
        Type::EmbeddedObject(_) => {
            let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site());
            quote! { #ident }
        }
    }
}

fn schedule_types(open_types: &mut Vec<SchemaUri>, closed_types: &HashSet<SchemaUri>, ty: &Type) {
    match ty {
        Type::Array(array_type) => {
            schedule_types(open_types, closed_types, array_type.item.as_ref())
        }
        Type::TypedObject(uri) => {
            if !closed_types.contains(uri) && !open_types.contains(uri) {
                open_types.push(uri.clone());
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

fn recursive_read_properties(
    properties: &mut HashMap<String, Property>,
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) {
    // First read properties from 'base' schemas
    let base_schema = schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|sub_schema| sub_schema.all_of.as_ref())
        .and_then(|all_of| all_of.first());
    if let Some(Schema::Object(base)) = base_schema {
        let base = schema.resolve(base);
        recursive_read_properties(properties, &base, open_types, closed_types);
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
            Type::Any => property.ty = handle_field(&field_schema, open_types, closed_types).unwrap(),
            _ => (),
        }

        schedule_types(open_types, closed_types, &property.ty);

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
        Type::EmbeddedObject(_) => unimplemented!(),
    }
}

fn get_raw_name(store: &SchemaStore, uri: &SchemaUri) -> String {
    if let Some(definition_name) = uri.definition_name() {
        return definition_name.to_lowercase();
    }

    let title = store
        .lookup(uri)
        .unwrap()
        .1
        .metadata
        .as_ref()
        .unwrap()
        .title
        .as_ref()
        .unwrap()
        .to_lowercase();

    // Remove module prefix from title
    let prefix_end = title.find(' ');
    if let Some(prefix_end) = prefix_end {
        if title[0..prefix_end].starts_with("khr_") {
            return title[(prefix_end + 1)..].to_string();
        }
    }

    title
}

fn generate_module_identifier(store: &SchemaStore, uri: &SchemaUri) -> Ident {
    let title = get_raw_name(store, uri);
    Ident::new(&title.to_case(Case::Snake), Span::call_site())
}
fn generate_struct_identifier(store: &SchemaStore, uri: &SchemaUri) -> Ident {
    let title = get_raw_name(store, uri);
    Ident::new(&title.to_case(Case::UpperCamel), Span::call_site())
}

fn generate_property_identifier(name: &str) -> Ident {
    Ident::new(
        &name.to_case(Case::Snake).replace("type", "ty"),
        Span::call_site(),
    )
}

/// Writes a rust type into a unique module with helper functions and type surrounding it
struct RustTypeWriter {
    embedded_types: Vec<TokenStream>,
    default_declarations: Vec<TokenStream>,
}

impl RustTypeWriter {
    fn new() -> Self {
        Self {
            embedded_types: Vec::new(),
            default_declarations: Vec::new(),
        }
    }
}

fn write_embedded_enum(property_name: &str, enumeration: &Enum, default: &Option<Value>) -> TokenStream{
    let rusty_enum_name = Ident::new(&property_name.to_case(Case::UpperCamel), Span::call_site());
    let enum_options = enumeration.options.iter().map(|option| {
        let identifier = Ident::new(
            &option.to_case(Case::UpperCamel).replace(&['/'], ""),
            Span::call_site(),
        );

        let is_default = match &default {
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

    let default_declaration = default
        .as_ref()
        .map(|_| quote! { #[derive(Default)] });
    quote! {
            #[derive(Serialize, Deserialize, Debug)]
            #default_declaration
            enum #rusty_enum_name{
                #(#enum_options),*
            }
        }
}

fn write_embedded_type(property_name: &str, ty: &Type, default: &Option<Value>, schema_store: &SchemaStore) -> Option<TokenStream>{
    match ty{
        Type::Array(array) => write_embedded_type(property_name, array.item.as_ref(), &None, schema_store),
        Type::EmbeddedObject(object_type) => Some(generate_structure(property_name, object_type, schema_store)),
        Type::Enum(enumeration) => Some(write_embedded_enum(property_name, enumeration, default)),
        _ => None
    }
}

fn write_property(
    schema_store: &SchemaStore,
    writer: &mut RustTypeWriter,
    name: &String,
    property: &Property,
) -> TokenStream {
    let rust_type = match (&property.ty, property.optional) {
        // Remove the Option for optional Vec's with a minimum length of 1
        // This way we can guarantee this invariant by telling serde to not serialize zero length vecs.
        (Type::Array(array_type), true)
            if array_type.min_length.is_some() && array_type.min_length.unwrap() == 1 =>
        {
            generate_rust_type(schema_store, &property.ty, name)
        }

        // Remove the Option for optional properties which have a default value specified.
        (_, true) if property.default.is_none() => {
            let rust_type: TokenStream = generate_rust_type(schema_store, &property.ty, name);
            quote! { Option::<#rust_type> }
        }
        _ => generate_rust_type(schema_store, &property.ty, name),
    };



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

    if let Some(embedded_type) = write_embedded_type(name, &property.ty, &property.default, schema_store){
        writer.embedded_types.push(embedded_type);
    }

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

fn read_typed_object(
    schema: &SchemaContext,
    open_list: &mut Vec<SchemaUri>,
    closed_list: &HashSet<SchemaUri>,
) -> ObjectType {
    let name = get_raw_name(schema.schema_store, schema.uri.as_ref().unwrap());
    let comment = schema.schema.metadata.as_ref().and_then(|metadata| metadata.description.clone());
    let mut properties = HashMap::new();
    recursive_read_properties(&mut properties, &schema, open_list, closed_list);
    ObjectType {
        name,
        prototype: ObjectPrototype{
            comment,
            properties
        }
    }
}

fn generate_structure(name: &str, object_prototype: &ObjectPrototype, schema_store: &SchemaStore) -> TokenStream {
    let mod_identifier = Ident::new(&name.to_case(Case::Snake), Span::call_site());
    let type_identifier = Ident::new(
        &name.to_case(Case::UpperCamel),
        Span::call_site(),
    );

    let mut property_tokens = Vec::new();
    let mut type_writer = RustTypeWriter::new();
    for (name, property) in object_prototype.properties.iter() {
        property_tokens.push(write_property(
            schema_store,
            &mut type_writer,
            name,
            property,
        ));
    }

    let doc = object_prototype
        .comment
        .as_ref()
        .map(|comment| quote! { #[doc=#comment]});
    let embedded_types = &type_writer.embedded_types;
    let default_declarations = &type_writer.default_declarations;

    quote! {
        mod #mod_identifier{
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};

            #(#embedded_types)*

            #[derive(Serialize, Deserialize, Debug)]
            #doc
            pub struct #type_identifier{
                #(#property_tokens),*
            }

            #(#default_declarations)*

        }
        pub use #mod_identifier::#type_identifier;
    }
}

fn generate_rust(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> TokenStream {
    let object_type = read_typed_object(schema, open_types, closed_types);
    generate_structure(&object_type.name, &object_type.prototype, schema.schema_store)
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
        let _ = extension_name
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
            SchemaStore::new_extension(&schemas_path, specification_schema, extension_name.clone());
        extension_schema_store.load();

        let mut extension_module = Vec::new();

        for (name, schema) in extension_schema_store.schemas.iter() {
            // If a schema ends with {Prefix}.ExtensionName.schema.json it represents the extension object with the extension name on that object
            let suffix_start = match name.find(&extension_schema_suffix) {
                Some(index) if index == name.len() - extension_schema_suffix.len() => index,
                _ => continue,
            };

            let base_object_name = &name[0..suffix_start];
            // TODO: Empty base object name seems to mean it applies to all
            if base_object_name.is_empty() {
                continue;
            }

            println!(
                "Extension {} has an extension on {}",
                &extension_name, &base_object_name
            );

            let base_object_module_ident = Ident::new(
                &base_object_name
                    .replace(".", " ")
                    .to_lowercase()
                    .to_case(Case::Snake),
                Span::call_site(),
            );
            let extension_doc = Some(format!(
                "The {extension_name} extension for {base_object_name}"
            ));

            let schema = extension_schema_store.make_context(&name.as_str().into());

            let mut open_types = Vec::new();
            let mut closed_types = HashSet::new();

            let comment = schema.schema.metadata.as_ref().and_then(|metadata| metadata.description.clone());
            let mut properties = HashMap::new();
            recursive_read_properties(&mut properties, &schema, &mut open_types, &closed_types);
            let prototype = ObjectPrototype{
                properties,
                comment
            };

            let structure = generate_structure("Extension", &prototype, schema.schema_store);
            extension_module.push(quote! {
               pub mod #base_object_module_ident{
                    #structure
                }
            });

            while !open_types.is_empty() {
                let uri = open_types.pop().unwrap();
                closed_types.insert(uri.clone());
                if !extension_schema_store.is_local_uri(&uri) {
                    continue;
                }

                let schema = extension_schema_store.make_context(&uri);
                extension_module.push(generate_rust(&schema, &mut open_types, &closed_types));
            }
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

fn main() {
    // Recreate the generated directory
    let generated_path = "gltf_for_rust\\src\\generated";
    ensure_empty_dir(generated_path);

    // Create the core specification schema store
    let mut specification_schema_store =
        SchemaStore::new_specification(&PathBuf::from("vendor\\gltf\\specification\\2.0\\schema"));
    specification_schema_store.load().unwrap();

    let mut generated_manifest = GeneratedManifest::new();
    load_extensions(
        &mut generated_manifest,
        "vendor\\gltf\\extensions\\2.0\\Khronos",
        generated_path,
        &specification_schema_store,
    )
    .unwrap();

    let output = File::create(format!("{generated_path}\\gltf.rs")).unwrap();
    let mut writer = BufWriter::new(output);

    // Collect root types:
    let mut closed_types = HashSet::new();
    let mut open_types = Vec::new();
    open_types.push(SchemaUri::from("glTF.schema.json"));

    while !open_types.is_empty() {
        let id = open_types.pop().unwrap();
        closed_types.insert(id.clone());
        let schema = specification_schema_store.make_context(&id);

        let rust = generate_rust(&schema, &mut open_types, &closed_types);

        let file: syn::File = syn::parse2(rust).unwrap();
        write!(&mut writer, "{}", prettyplease::unparse(&file));
    }

    write_root_module(generated_path, &generated_manifest);
}
