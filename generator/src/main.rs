use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Write;
use std::vec::Vec;

use anyhow::Context;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde_json::Value;

use crate::module_builder::{ModuleBuilder, TypeDescription};
use crate::schema::{Schema, SchemaContext, SchemaResolver, SchemaStore, SchemaStoreMeta};
use crate::schema_uri::SchemaUri;

mod module_builder;
mod naming;
mod schema;
mod schema_uri;
mod type_deduction;
mod codegen;
mod module_tree;

pub struct Enum {
    options: Vec<String>,
}

pub struct ArrayType {
    pub min_length: Option<usize>,
    pub item: Box<Type>,
}

pub struct FixedArrayType {
    pub item: Box<Type>,
    pub length: u32,
}

pub enum Type {
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

pub struct ObjectPrototype {
    pub comment: Option<String>,
    pub properties: Vec<Property>,
}

pub struct ObjectType {
    pub name: String,
    pub prototype: ObjectPrototype,
}


pub struct Property {
    pub name: String,
    pub ty: Type,
    pub optional: bool,
    pub default: Option<Value>,
    pub comment: Option<String>,
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

    fn read_description<'a>(
        schema_resolver: &'a SchemaResolver,
        context: &SchemaContext,
        schema: &'a Schema,
    ) -> Option<&'a str> {
        if let Some(detailed_description) = schema.detailed_description() {
            return Some(detailed_description);
        }

        if let Some(description) = schema.description() {
            return Some(description);
        }

        if let Some((context, schema)) = schema.reference().and_then(|reference| {
            schema_resolver.resolve(&SchemaUri::from_str(reference), Some(context.uri()))
        }) {
            return Self::read_description(schema_resolver, &context, schema);
        }

        None
    }

    fn recursive_read_properties(
        &mut self,
        resolver: &SchemaResolver,
        context: &SchemaContext,
        schema: &Schema,
    ) -> anyhow::Result<()> {
        // TODO: Ensure proper handling of compound schemas, right now we assume 'inheritance'

        // Read properties from reference schema
        if let Some((context, schema)) = schema.reference().and_then(|reference| {
            resolver.resolve(&SchemaUri::from_str(reference), Some(context.uri()))
        }) {
            self.recursive_read_properties(resolver, &context, schema)?;
        }

        // First read properties from 'base' schemas
        if let Ok((context, schema)) = schema.all_of(context).exactly_one() {
            self.recursive_read_properties(resolver, &context, schema)?;
        }

        // Then add our own properties
        for (context, name, field_schema) in schema.properties(context) {
            let property = self.find_or_add(name);
            if let Type::Any = property.ty {
                property.ty = type_deduction::handle_field(resolver, &context, field_schema)
                    .with_context(|| {
                        format!("failed to deduce field type for property \"{name}\"")
                    })?;
            }

            if property.comment.is_none() {
                property.comment =
                    Self::read_description(resolver, &context, field_schema).map(String::from);
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

        let mut extension_schema_store = SchemaStore::read(
            SchemaStoreMeta::Extension(extension_name.clone()),
            &schemas_path.to_string_lossy(),
        )
            .unwrap();
        let resolver = SchemaResolver::extension(specification_schema, &extension_schema_store);

        let mut specification_builder = ModuleBuilder::new(
            generated_path,
            &extension_name,
            &resolver,
            &extension_schema_store,
        );

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

            let base_module_name = naming::generate_base_module_identifier(base_object_name).to_string();


            specification_builder.push(TypeDescription {
                schema: uri.clone(),
                module_path_override: Some(vec![base_module_name]),
                name_override: Some(String::from("Extension")),
                extension: Some(extension_name.clone()),
            });
        }

        specification_builder.traverse();
        specification_builder.generate().unwrap();
        generated_manifest
            .extension_modules
            .push(extension_module_name);
    }

    Ok(())
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

fn main() {
    const SPECIFICATION_FOLDER: &str = "vendor/gltf/specification/2.0/schema";
    const KHRONOS_EXTENSIONS_FOLDER: &str = "vendor/gltf/extensions/2.0/Khronos";
    const VENDOR_EXTENSIONS_FOLDER: &str = "vendor/gltf/extensions/2.0/Vendor";

    // Recreate the generated directory
    const OUTPUT_BASE: &str = "gltf_for_rust/src/generated";

    // Create the core specification schema store
    let specification_schema_store =
        SchemaStore::read(SchemaStoreMeta::Core, SPECIFICATION_FOLDER).unwrap();
    let specification_resolver = SchemaResolver::specification(&specification_schema_store);

    let mut specification_builder = ModuleBuilder::new(
        OUTPUT_BASE,
        "gltf",
        &specification_resolver,
        &specification_schema_store,
    );
    specification_builder.push(TypeDescription {
        schema: SchemaUri::from_str("glTF.schema.json"),
        module_path_override: None,
        name_override: None,
        extension: None,
    });
    specification_builder.traverse();
    specification_builder.generate().unwrap();

    let mut generated_manifest = GeneratedManifest::new();
    load_extensions(
        &mut generated_manifest,
        KHRONOS_EXTENSIONS_FOLDER,
        OUTPUT_BASE,
        &specification_schema_store,
    )
        .unwrap();

    load_extensions(
        &mut generated_manifest,
        VENDOR_EXTENSIONS_FOLDER,
        OUTPUT_BASE,
        &specification_schema_store,
    )
        .unwrap();

    codegen::write_root_module(OUTPUT_BASE, &generated_manifest);
}
