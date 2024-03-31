use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, read_dir};
use std::io::{BufReader, ErrorKind};
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

impl SchemaUri{
    pub fn definition_name(&self) -> Option<&str>{
        const DEFINITION_NS : &str = "/definitions/";
        self.fragment.as_ref().and_then(|fragment| {
          if fragment.starts_with(DEFINITION_NS){
              Some(&fragment[DEFINITION_NS.len()..])
          }else{
              None
          }
        })
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
            let mut reference = schema.reference.as_ref().unwrap().clone();

            // If the reference is a local URI replace with the full URI instead
            if reference.starts_with('#'){
                reference = self.uri.as_ref().unwrap().path.as_ref().unwrap().to_owned() + &reference;
            }

            return SchemaContext {
                schema_store: self.schema_store,
                uri: Some(reference.as_str().into()),
                schema: self
                    .schema_store
                    .lookup(&reference.as_str().into())
                    .unwrap().1
            };
        }

        SchemaContext {
            schema_store: self.schema_store,
            uri: self.uri.clone(),
            schema,
        }
    }
}

pub enum SchemaType{
    Specification,
    Extension(String)
}

pub struct SchemaStore<'a> {
    pub ty: SchemaType,
    folder: PathBuf,
    pub schemas: HashMap<String, RootSchema>,
    base: Option<&'a SchemaStore<'a>>,
}

fn lookup_fragment<'a>(schema: &'a RootSchema, uri: &SchemaUri) -> Option<&'a SchemaObject>{
    if let Some(definition_name) = uri.definition_name(){
       let found_schema = schema.definitions.get(definition_name).unwrap();
        match found_schema{
            Schema::Object(object) => Some(object),
            _ => unreachable!()
        }
    }else{
        None
    }
}

impl<'a> SchemaStore<'a> {
    pub fn load(folder: &Path, base: Option<&'a SchemaStore>) -> Result<Self, Box<dyn Error>> {
        let mut store = Self{
            ty: SchemaType::Specification,
            schemas: HashMap::new(),
            folder: PathBuf::from(folder),
            base
        };

        let dir = match read_dir(folder){
            Ok(dir) => dir,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(store),
            Err(e) => return Err(Box::new(e)),
        };

        for entry in dir.into_iter().filter_map(Result::ok){
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with("schema.json"){
                store.read(&file_name)?;
            }
        }

        Ok(store)
    }

    pub fn new_extension(base: &'a SchemaStore<'a>, folder: &Path, extension_name: String) -> Self {
        Self {
            ty: SchemaType::Extension(extension_name),
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

    pub fn is_local_uri(&self, uri: &SchemaUri) -> bool{
        self.schemas.contains_key(uri.path.as_ref().unwrap())
    }

    pub fn make_context(&self, uri: &SchemaUri) -> SchemaContext{
        let root_schema = self.schemas.get(uri.path.as_ref().unwrap()).unwrap();

        SchemaContext{
            schema_store: self,
            uri: Some(uri.clone()),
            schema: &root_schema.schema,
        }
    }

    pub fn lookup(&self, uri: &SchemaUri) -> Option<(&SchemaStore, &SchemaObject)> {
        // Try in base first
        if let Some(base) = self.base {
            match base.lookup(uri) {
                Some(object) => return Some(object),
                _ => (),
            }
        }

        if let Some(root) = self.schemas.get(uri.path.as_ref().unwrap()){
            if let Some(fragment) = uri.fragment.as_ref(){
                lookup_fragment(root, uri).map(|fragment| (self, fragment))
            }else{
                Some((self, &root.schema))
            }
        }else{
            None
        }
    }
}
