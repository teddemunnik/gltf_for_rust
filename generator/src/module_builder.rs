use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Context;

use crate::{codegen, naming, ObjectPrototype, ObjectType, PropertyListBuilder, Type};
use crate::schema::{Schema, SchemaContext, SchemaResolver, SchemaStore};
use crate::schema_uri::SchemaUri;

pub struct TypeDescription {
    pub schema: SchemaUri,
    pub module_path_override: Option<Vec<String>>,
    pub name_override: Option<String>,
    pub extension: Option<String>,
}

pub struct ResolvedType {
    pub module_path: Vec<String>,
    pub name: String,
    pub prototype: ObjectPrototype,
    pub extension: Option<String>,
}

pub struct ModuleBuilder<'a> {
    pub output_base: String,
    pub name: String,
    pub types: BTreeMap<SchemaUri, Arc<ResolvedType>>,
    open_list: Vec<TypeDescription>,
    store: &'a SchemaStore,
    pub resolver: &'a SchemaResolver<'a>,
}


fn read_typed_object(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> ObjectType {
    let name = naming::get_canonical_name(context, schema).unwrap();
    let comment = schema.description().map(|desc| desc.to_string());
    let mut properties = PropertyListBuilder::new();
    properties
        .recursive_read_properties(resolver, context, schema)
        .with_context(|| format!("Failed to read properties for schema {}", context.uri()))
        .unwrap();
    ObjectType {
        name,
        prototype: ObjectPrototype {
            comment,
            properties: properties.properties,
        },
    }
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(
        output_base: &str,
        name: &str,
        resolver: &'a SchemaResolver,
        store: &'a SchemaStore,
    ) -> Self {
        Self {
            output_base: String::from(output_base),
            name: String::from(name),
            types: BTreeMap::new(),
            resolver,
            store,
            open_list: Vec::new(),
        }
    }
    pub fn push(&mut self, ty: TypeDescription) {
        self.open_list.push(ty);
    }

    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::TypedObject(schema) => {
                if self.types.contains_key(schema)
                    || self.open_list.iter().any(|item| item.schema.eq(schema))
                {
                    return;
                }

                self.open_list.push(TypeDescription {
                    schema: schema.clone(),
                    module_path_override: None,
                    name_override: None,
                    extension: None,
                })
            }
            Type::Array(array) => self.visit_type(&array.item),
            _ => (),
        }
    }

    pub fn traverse(&mut self) {
        while let Some(ty) = self.open_list.pop() {
            if !self.store.is_local_uri(&ty.schema) {
                continue;
            }

            let (context, schema) = self.resolver.resolve(&ty.schema, None).unwrap();

            // Create the object prototype
            let object_type = read_typed_object(self.resolver, &context, schema);

            // Schedule nested types for generation
            for property in object_type.prototype.properties.iter() {
                self.visit_type(&property.ty);
            }

            self.types.insert(
                ty.schema.clone(),
                Arc::new(
                    ResolvedType {
                        name: ty.name_override.unwrap_or(object_type.name),
                        module_path: ty.module_path_override.unwrap_or(vec![]),
                        prototype: object_type.prototype,
                        extension: ty.extension,
                    }),
            );
        }
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        codegen::write_module(self)
    }
}
