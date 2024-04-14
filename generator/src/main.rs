use std::{fs, fs::File, io::BufWriter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_dir;
use std::io::Write;
use std::path::PathBuf;
use std::vec::Vec;

use anyhow::Context;
use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde_json::Value;
use thiserror::Error;

use crate::naming::{
    generate_enum_type_identifier, generate_option_identifier, generate_property_identifier,
};
use crate::schema::{InstanceType, Schema, SchemaContext, SchemaResolver, SchemaStore, SchemaStoreMeta};
use crate::schema_uri::SchemaUri;

mod naming;
mod schema;
mod schema_uri;
mod type_deduction;

fn plural_to_singular(maybe_plural: &str) -> String {
    if let Some(singular) = maybe_plural.strip_suffix("ies") {
        format!("{}y", singular)
    } else if let Some(singular) = maybe_plural.strip_suffix('s') {
        String::from(singular)
    } else {
        String::from(maybe_plural)
    }
}

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to open schema {path}: {inner}")]
    FailedToOpenSchema {
        path: PathBuf,
        inner: Box<dyn std::error::Error>,
    },
    #[error("Unhandled instance type")]
    UnhandledInstanceType(Vec<InstanceType>),
    #[error("Unhandled array item type")]
    UnhandledArrayItemType(Option<Vec<Schema>>),
}

struct Enum {
    options: Vec<String>,
}

struct ArrayType {
    min_length: Option<usize>,
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
    EmbeddedObject {
        name: Option<String>,
        prototype: ObjectPrototype,
    },
    String,
    Boolean,
    Number,
    Integer,
    Enum(Enum),
    MapOfObjects,
}

struct ObjectPrototype {
    comment: Option<String>,
    properties: Vec<Property>,
}

struct ObjectType {
    name: String,
    prototype: ObjectPrototype,
}

fn generate_named_type_path(resolver: &SchemaResolver, uri: &SchemaUri) -> TokenStream {
    let (context, schema) = resolver.resolve(uri, None).unwrap();

    let name = naming::get_canonical_name(&context, &schema).unwrap();
    let type_name = Ident::new(&name, Span::call_site());

    match context.meta() {
        SchemaStoreMeta::Core => quote! { crate::generated::gltf::#type_name },
        SchemaStoreMeta::Extension(extension) => {
            let ident = naming::generate_base_module_identifier(extension);
            quote! { crate::generated::#ident::#type_name}
        }
    }
}

fn generate_rust_type(resolver: &SchemaResolver, ty: &Type, field_name: &String) -> TokenStream {
    match ty {
        Type::Any => quote! { serde_json::Value },
        Type::Array(array_type) => {
            let item_rust_type = generate_rust_type(resolver, &array_type.item, field_name);
            quote! { Vec::< #item_rust_type > }
        }
        Type::FixedArray(array_type) => {
            let fixed_length = array_type.length as usize;
            let rust_item_type = generate_rust_type(resolver, &array_type.item, field_name);
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
        Type::TypedObject(uri) => generate_named_type_path(resolver, uri),
        Type::MapOfObjects => quote! { Map<String, Value> },
        Type::EmbeddedObject { name, prototype } => {
            let ident = Ident::new(
                &name
                    .clone()
                    .unwrap_or_else(|| plural_to_singular(field_name.as_str()))
                    .to_case(Case::UpperCamel),
                Span::call_site(),
            );
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
    name: String,
    ty: Type,
    optional: bool,
    default: Option<Value>,
    comment: Option<String>,
}

struct PropertyListBuilder {
    by_name: HashMap<String, usize>,
    properties: Vec<Property>,
}

impl PropertyListBuilder {
    fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            properties: Vec::new(),
        }
    }

    fn find_or_add(&mut self, name: &str) -> &mut Property {
        if let Some(existing_id) = self.by_name.get(name) {
            &mut self.properties[*existing_id]
        } else {
            self.by_name.insert(name.to_string(), self.properties.len());
            self.properties.push(Property {
                name: name.to_string(),
                ty: Type::Any,
                optional: true,
                comment: None,
                default: None,
            });
            self.properties.last_mut().unwrap()
        }
    }

    fn read_description<'a>(schema_resolver: &'a SchemaResolver, context: &SchemaContext, schema: &'a Schema) -> Option<&'a str> {
        if let Some(detailed_description) = schema.detailed_description() {
            return Some(detailed_description);
        }

        if let Some(description) = schema.description() {
            return Some(description);
        }

        if let Some((context, schema)) = schema.reference().and_then(|reference| schema_resolver.resolve(&SchemaUri::from_str(reference), Some(context.uri()))) {
            return Self::read_description(schema_resolver, &context, schema);
        }

        None
    }

    fn recursive_read_properties(
        &mut self,
        resolver: &SchemaResolver,
        context: &SchemaContext,
        schema: &Schema,
        open_types: &mut Vec<SchemaUri>,
        closed_types: &HashSet<SchemaUri>,
    ) -> anyhow::Result<()> {
        // TODO: Ensure proper handling of compound schemas, right now we assume 'inheritance'

        // Read properties from reference schema
        if let Some((context, schema)) = schema.reference().and_then(|reference| resolver.resolve(&SchemaUri::from_str(reference), Some(context.uri()))) {
            self.recursive_read_properties(resolver, &context, schema, open_types, closed_types)?;
        }

        // First read properties from 'base' schemas
        if let Ok((context, schema)) = schema.all_of(context).exactly_one() {
            self.recursive_read_properties(resolver, &context, schema, open_types, closed_types)?;
        }

        // Then add our own properties
        for (context, name, field_schema) in schema.properties(context) {
            let property = self.find_or_add(name);
            if let Type::Any = property.ty {
                property.ty =
                    type_deduction::handle_field(resolver, &context, field_schema, open_types, closed_types)
                        .with_context(|| {
                            format!("failed to deduce field type for property \"{name}\"")
                        })?;
            }

            schedule_types(open_types, closed_types, &property.ty);

            if property.comment.is_none() {
                property.comment = Self::read_description(resolver, &context, field_schema).map(String::from);
            }

            if property.default.is_none() {
                property.default = field_schema.default().cloned();
            }

            if schema.required().iter().contains(&name.to_string()) {
                property.optional = false;
            }
        }
        Ok(())
    }
}

fn generate_default_value_token(ty: &Type, default: &Value, field_name: &String) -> anyhow::Result<TokenStream> {
    Ok(match ty {
        Type::Any => unimplemented!(),
        Type::Array(_) => quote! {{ Vec::default(); }},
        Type::FixedArray(array) => {
            let array_items: Vec<TokenStream> = default
                .as_array()
                .context("Expected the default value to be an array")?.iter().map(|value| generate_default_value_token(&array.item, value, field_name)).try_collect()?;


            quote! { [ #(#array_items),* ]}
        }
        Type::Boolean => {
            let value = default.as_bool().context("Expected the default value to be a bool")?;
            quote! { #value }
        }
        Type::Enum(_) => {
            let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site());
            quote! { #ident::default() }
        }
        Type::Integer => {
            let integer: i64 = default.as_i64().context("Unexpected the default to be an integer")?;
            quote! { #integer }
        }
        Type::MapOfObjects => quote! { Map<String, Value>::default() },
        Type::Number => {
            let number: f64 = default.as_f64().unwrap();
            quote! { #number }
        }
        Type::String => unimplemented!(),
        Type::TypedObject(_) => unimplemented!(),
        Type::EmbeddedObject { name, prototype } => unimplemented!(),
    })
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

fn write_embedded_enum(
    property_name: &str,
    enumeration: &Enum,
    default: &Option<Value>,
) -> TokenStream {
    let enum_identifier = generate_enum_type_identifier(property_name);
    let enum_options = enumeration.options.iter().map(|option| {
        let identifier = generate_option_identifier(option);

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

    let default_declaration = default.as_ref().map(|_| quote! { #[derive(Default)] });
    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #default_declaration
        pub enum #enum_identifier{
            #(#enum_options),*
        }
    }
}

fn write_embedded_type(
    property_name: &str,
    ty: &Type,
    default: &Option<Value>,
    resolver: &SchemaResolver,
) -> anyhow::Result<Option<TokenStream>> {
    Ok(match ty {
        Type::Array(array) => {
            write_embedded_type(property_name, array.item.as_ref(), &None, resolver)?
        }
        Type::EmbeddedObject { name, prototype } => Some(generate_structure(
            &name
                .clone()
                .unwrap_or_else(|| plural_to_singular(property_name)),
            prototype,
            resolver,
        ).with_context(|| format!("Failed to generate embedded type {}", name.as_ref().unwrap()))?),
        Type::Enum(enumeration) => Some(write_embedded_enum(property_name, enumeration, default)),
        _ => None,
    })
}

fn write_property(
    resolver: &SchemaResolver,
    writer: &mut RustTypeWriter,
    property: &Property,
) -> anyhow::Result<TokenStream> {
    let rust_type = match (&property.ty, property.optional) {
        // Remove the Option for optional Vec's with a minimum length of 1
        // This way we can guarantee this invariant by telling serde to not serialize zero length vecs.
        (Type::Array(array_type), true)
        if array_type.min_length.is_some() && array_type.min_length.unwrap() == 1 =>
            {
                generate_rust_type(resolver, &property.ty, &property.name)
            }

        (_, true) => {
            let rust_type: TokenStream = generate_rust_type(resolver, &property.ty, &property.name);
            quote! { Option::<#rust_type> }
        }
        _ => generate_rust_type(resolver, &property.ty, &property.name),
    };

    let property_identifier = generate_property_identifier(&property.name);
    let property_identifier_name = property_identifier.to_string();

    if let Some(embedded_type) =
        write_embedded_type(&property.name, &property.ty, &property.default, resolver)?
    {
        writer.embedded_types.push(embedded_type);
    }

    let default_declaration = match property.optional {
        true => Some(quote! { #[serde(default)]}),
        false => None
    };

    // If the property identifier is different from the one in the spec we need to add a serde
    // rename to make it match the spec.
    let rename_declaration =
        if property_identifier_name.partial_cmp(&property.name) != Some(Ordering::Equal) {
            let name = &property.name;
            Some(quote![#[serde(rename = #name)]])
        } else {
            None
        };

    let docstring = property.comment.as_ref().map(|x| quote! { #[doc=#x] });
    Ok(quote! {
        #rename_declaration
        #default_declaration
        #docstring
        pub #property_identifier: #rust_type
    })
}

fn read_typed_object(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
    open_list: &mut Vec<SchemaUri>,
    closed_list: &HashSet<SchemaUri>,
) -> ObjectType {
    let name = naming::get_canonical_name(context, schema).unwrap();
    let comment = schema.description().map(|desc| desc.to_string());
    let mut properties = PropertyListBuilder::new();
    properties
        .recursive_read_properties(resolver, context, schema, open_list, closed_list)
        .with_context(|| {
            format!(
                "Failed to read properties for schema {}",
                context.uri()
            )
        })
        .unwrap();
    ObjectType {
        name,
        prototype: ObjectPrototype {
            comment,
            properties: properties.properties,
        },
    }
}

fn generate_structure(
    name: &str,
    object_prototype: &ObjectPrototype,
    resolver: &SchemaResolver,
) -> anyhow::Result<TokenStream> {
    let mod_identifier = &naming::generate_property_identifier(name);
    let type_identifier = naming::generate_type_identifier(name);

    let mut property_tokens = Vec::new();
    let mut type_writer = RustTypeWriter::new();
    for property in object_prototype.properties.iter() {
        property_tokens.push(write_property(resolver, &mut type_writer, property).with_context(|| format!("failed to write property {}", property.name))?)
    }

    let doc = object_prototype
        .comment
        .as_ref()
        .map(|comment| quote! { #[doc=#comment]});
    let embedded_types = &type_writer.embedded_types;
    let default_declarations = &type_writer.default_declarations;

    // Trait implementation if the object supports extensions
    let gltf_object_trait = if object_prototype
        .properties
        .iter()
        .any(|property| property.name.eq("extensions"))
    {
        Some(quote! {
            impl crate::GltfObject for #type_identifier{
                fn extensions(&self) -> &Option<Map<String, Value>>{
                    &self.extensions
                }
            }
        })
    } else {
        None
    };

    Ok(quote! {
        mod #mod_identifier{
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};

            #(#embedded_types)*

            #[derive(Serialize, Deserialize, Debug)]
            #doc
            pub struct #type_identifier{
                #(#property_tokens),*
            }

            #gltf_object_trait

            #(#default_declarations)*

        }
        pub use #mod_identifier::#type_identifier;

    })
}

fn generate_rust(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<TokenStream> {
    let object_type = read_typed_object(resolver, context, schema, open_types, closed_types);
    generate_structure(&object_type.name, &object_type.prototype, resolver).with_context(|| format!("Failed to generate rust for schema {}", context.uri()))
}

#[allow(unused)]
fn load_extensions(
    generated_manifest: &mut GeneratedManifest,
    extensions_path: &str,
    generated_path: &str,
    specification_schema: &SchemaStore,
) -> anyhow::Result<()> {
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

        let mut extension_schema_store = SchemaStore::read(SchemaStoreMeta::Extension(extension_name.clone()), &schemas_path.to_string_lossy()).unwrap();
        let resolver = SchemaResolver::extension(&specification_schema, &extension_schema_store);

        let mut extension_module = Vec::new();
        let mut open_types = Vec::new();
        let mut closed_types = HashSet::new();

        for (context, schema) in extension_schema_store.schemas() {
            // If a schema ends with {Prefix}.ExtensionName.schema.json it represents the extension object with the extension name on that object
            let uri = context.uri();

            let base_object_name = match uri.path.strip_suffix(&extension_schema_suffix) {
                Some(base_object_name) => base_object_name,
                None => continue,
            };

            // TODO: Empty base object name seems to mean it applies to all
            if base_object_name.is_empty() {
                continue;
            }

            println!(
                "Extension {} has an extension on {}",
                &extension_name, &base_object_name
            );

            let base_object_module_ident =
                naming::generate_base_module_identifier(base_object_name);
            let extension_doc = Some(format!(
                "The {extension_name} extension for {base_object_name}"
            ));

            let comment = schema.description().map(|desc| desc.to_string());
            let mut properties = PropertyListBuilder::new();
            properties.recursive_read_properties(&resolver, &context, schema, &mut open_types, &closed_types).with_context(|| format!("Failed to read properties for extension {extension_name} on base object {base_object_name}")).unwrap();
            let prototype = ObjectPrototype {
                properties: properties.properties,
                comment,
            };

            let structure = generate_structure("Extension", &prototype, &resolver).with_context(|| format!("Failed to generate structure {}", context.uri()))?;
            extension_module.push(quote! {
               pub mod #base_object_module_ident{
                    #structure

                    impl crate::GltfExtension for Extension{
                        fn extension_name() -> &'static str{
                            #extension_name
                        }
                    }
                }
            });
        }

        while let Some(uri) = open_types.pop() {
            closed_types.insert(uri.clone());
            if !extension_schema_store.is_local_uri(&uri) {
                continue;
            }

            let (context, schema) = resolver.resolve(&uri, None).unwrap();

            extension_module.push(generate_rust(&resolver, &context, &schema, &mut open_types, &closed_types)?);
        }

        let output = File::create(format!("{generated_path}/{extension_module_name}.rs")).unwrap();
        let mut writer = BufWriter::new(output);

        let rust = quote! {
            #![allow(clippy::all, unused_imports)]

            #(#extension_module)*
        };

        let rust_file: syn::File = syn::parse2(rust).unwrap();
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
            for entry in dir.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
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
    let output = File::create(format!("{generated_path}/mod.rs")).unwrap();
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
    let generated_path = "gltf_for_rust/src/generated";
    ensure_empty_dir(generated_path);

    const SPECIFICATION_FOLDER: &str = "vendor/gltf/specification/2.0/schema";

    // Create the core specification schema store
    let mut specification_schema_store = SchemaStore::read(SchemaStoreMeta::Core, SPECIFICATION_FOLDER).unwrap();

    // Collect root types:
    let mut closed_types = HashSet::new();
    let mut open_types = Vec::new();

    let mut types = Vec::new();
    open_types.push(SchemaUri::from_str("glTF.schema.json"));

    let specification_resolver = SchemaResolver::specification(&specification_schema_store);

    while let Some(uri) = open_types.pop() {
        closed_types.insert(uri.clone());
        let (context, schema) = specification_resolver.resolve(&uri, None).unwrap();
        types.push(generate_rust(&specification_resolver, &context, &schema, &mut open_types, &closed_types).unwrap());
    }

    let rust = quote! {
        #![allow(clippy::all, unused_imports)]

        #(#types)*
    };

    let file: syn::File = syn::parse2(rust).unwrap();
    let output = File::create(format!("{generated_path}/gltf.rs")).unwrap();
    let mut writer = BufWriter::new(output);
    write!(writer, "{}", prettyplease::unparse(&file)).unwrap();

    let mut generated_manifest = GeneratedManifest::new();
    load_extensions(
        &mut generated_manifest,
        "vendor/gltf/extensions/2.0/Khronos",
        generated_path,
        &specification_schema_store,
    )
        .unwrap();

    load_extensions(
        &mut generated_manifest,
        "vendor/gltf/extensions/2.0/Vendor",
        generated_path,
        &specification_schema_store,
    )
        .unwrap();

    write_root_module(generated_path, &generated_manifest);
}
