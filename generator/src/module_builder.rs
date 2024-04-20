use std::collections::BTreeMap;

use crate::{codegen, ObjectPrototype, read_typed_object, Type};
use crate::schema::{SchemaResolver, SchemaStore};
use crate::schema_uri::SchemaUri;

pub struct TypeDescription {
    pub schema: SchemaUri,
    pub name_override: Option<String>,
    pub extension: Option<String>,
}

pub struct ResolvedType {
    pub name: String,
    pub prototype: ObjectPrototype,
    pub extension: Option<String>,
}

pub struct ModuleBuilder<'a> {
    pub output_base: String,
    pub name: String,
    pub types: BTreeMap<SchemaUri, ResolvedType>,
    open_list: Vec<TypeDescription>,
    store: &'a SchemaStore,
    pub resolver: &'a SchemaResolver<'a>,
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
                ResolvedType {
                    name: ty.name_override.unwrap_or(object_type.name),
                    prototype: object_type.prototype,
                    extension: ty.extension,
                },
            );
        }
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        codegen::write_module(self)
    }
}
