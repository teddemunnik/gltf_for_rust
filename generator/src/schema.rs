use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, read_dir};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use schemars::schema::{RootSchema, SchemaObject};
use crate::{MyError};

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
    pub schemas: HashMap<String, RootSchema>,
    base: Option<&'a SchemaStore<'a>>,
}

impl<'a> SchemaStore<'a> {
    pub fn load(folder: &Path, base: Option<&'a SchemaStore>) -> Result<Self, Box<dyn Error>> {
        let mut store = Self{
            schemas: HashMap::new(),
            folder: PathBuf::from(folder),
            base
        };

        for entry in read_dir(&folder).unwrap().into_iter().filter_map(Result::ok){
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with("schema.json"){
                store.read(&file_name)?;
            }
        }

        Ok(store)
    }

    #[allow(unused)]
    pub fn new_extension(base: &'a SchemaStore<'a>, folder: &Path) -> Self {
        Self {
            folder: PathBuf::from(folder),
            base: Some(base),
            schemas: HashMap::new(),
        }
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
        let root_schema =
            serde_json::from_reader(reader).map_err(|e| MyError::FailedToOpenSchema {
                path: full_path.clone(),
                inner: Box::new(e),
            })?;

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
