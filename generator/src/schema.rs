use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, read_dir};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use convert_case::Casing;
use schemars::schema::{RootSchema, Schema, SchemaObject};
use crate::{MyError};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SchemaUri{
    pub path: Option<String>,
    pub fragment: Option<String>,
}

impl From<&str> for SchemaUri{
    fn from(value: &str) -> Self {
        let fragment_index = value.find('#');
        match fragment_index{
            Some(index) if index != 0 => SchemaUri{ path: Some(value[..index].to_string()), fragment: Some(value[(index+1)..].to_string())},
            Some(index) => SchemaUri{ path: None, fragment: Some(value[(index+1)..].to_string())},
            None => SchemaUri{ path: Some(value.to_string()), fragment: None }
        }
    }
}

#[derive(Clone)]
pub struct SchemaContext<'a> {
    pub schema_store: &'a SchemaStore<'a>,
    pub schema: &'a SchemaObject,
    pub uri: Option<SchemaUri>,
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
                uri: Some(reference.as_str().into()),
                schema: self
                    .schema_store
                    .lookup(&reference.as_str().into())
                    .unwrap(),
            };
        }

        SchemaContext {
            schema_store: self.schema_store,
            uri: self.uri.clone(),
            schema,
        }
    }
}

pub struct SchemaStore<'a> {
    folder: PathBuf,
    pub schemas: HashMap<String, RootSchema>,
    base: Option<&'a SchemaStore<'a>>,
}

fn lookup_definition_fragment<'a>(namespace: &str, fragment: &str, schema: &'a RootSchema) -> Option<&'a SchemaObject>{
    if fragment.starts_with(namespace){
        let definition_name = &fragment[namespace.len()..];
        let found_schema = schema.definitions.get(definition_name);
        found_schema.map(|schema| match schema{
            Schema::Object(object) => object,
            _ => unreachable!()
        })
    }
    else{
        None
    }
}

fn lookup_fragment<'a>(schema: &'a RootSchema, fragment: &str) -> Option<&'a SchemaObject>{
    lookup_definition_fragment("/definitions/", fragment, schema).or_else(|| lookup_definition_fragment("/$defs/", fragment, schema))
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

    pub fn make_context(&self, uri: &SchemaUri) -> SchemaContext{
        let root_schema = self.schemas.get(uri.path.as_ref().unwrap()).unwrap();

        SchemaContext{
            schema_store: self,
            uri: Some(uri.clone()),
            schema: &root_schema.schema,
        }
    }

    pub fn lookup(&self, uri: &SchemaUri) -> Option<&SchemaObject> {
        // Try in base first
        if let Some(base) = self.base {
            match base.lookup(uri) {
                Some(object) => return Some(object),
                _ => (),
            }
        }

        let root = self.schemas.get(uri.path.as_ref().unwrap()).unwrap();

        if let Some(fragment) = uri.fragment.as_ref(){
            lookup_fragment(root, &fragment)
        }else{
            Some(&root.schema)
        }
    }
}
