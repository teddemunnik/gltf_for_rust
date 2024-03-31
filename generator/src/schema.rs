use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use schemars::schema::{RootSchema, SchemaObject};
use schemars::visit::{visit_root_schema, visit_schema_object, Visitor};
use crate::{MyError};


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


#[derive(Clone)]
pub struct SchemaContext<'a> {
    pub schema_store: &'a SchemaStore<'a>,
    pub schema: &'a SchemaObject,
    pub id: Option<String>,
}

impl<'a> SchemaContext<'a> {
    pub fn resolve<'b, 'c>(&'b self, schema: &'b SchemaObject) -> SchemaContext<'c>
        where
            'b: 'c,
    {
        if schema.is_ref() {
            let reference = schema.reference.as_ref().unwrap();
            return SchemaContext {
                schema_store: self.schema_store,
                id: Some(reference.clone()),
                schema: self
                    .schema_store
                    .lookup(reference)
                    .unwrap(),
            };
        }

        SchemaContext {
            schema_store: self.schema_store,
            id: self.id.clone(),
            schema,
        }
    }
}

pub struct SchemaStore<'a> {
    folder: PathBuf,
    schemas: HashMap<String, RootSchema>,
    pub roots: Vec<String>,
    base: Option<&'a SchemaStore<'a>>,
}

impl<'a> SchemaStore<'a> {
    pub fn new_root(folder: &Path) -> Self {
        Self {
            folder: PathBuf::from(folder),
            base: None,
            roots: Vec::new(),
            schemas: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn new_extension(base: &'a SchemaStore<'a>, folder: &Path) -> Self {
        Self {
            folder: PathBuf::from(folder),
            base: Some(base),
            roots: Vec::new(),
            schemas: HashMap::new(),
        }
    }

    pub fn read_root(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        let result = self.read(id);
        self.roots.push(id.into());
        result
    }

    pub fn read(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
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

    pub fn make_context(&self, name: &str) -> SchemaContext{
        let root_schema = self.schemas.get(name).unwrap();

        SchemaContext{
            schema_store: self,
            id: Some(name.to_string()),
            schema: &root_schema.schema,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SchemaObject> {
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
