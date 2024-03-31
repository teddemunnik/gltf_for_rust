use crate::MyError;
use schemars::schema::{RootSchema, Schema, SchemaObject};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{BufReader, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SchemaUri {
    pub path: Option<String>,
    pub fragment: Option<String>,
}

impl From<&str> for SchemaUri {
    fn from(value: &str) -> Self {
        let fragment_index = value.find('#');
        match fragment_index {
            Some(index) if index != 0 => SchemaUri {
                path: Some(value[..index].to_string()),
                fragment: Some(value[(index + 1)..].to_string()),
            },
            Some(index) => SchemaUri {
                path: None,
                fragment: Some(value[(index + 1)..].to_string()),
            },
            None => SchemaUri {
                path: Some(value.to_string()),
                fragment: None,
            },
        }
    }
}

impl SchemaUri {
    pub fn definition_name(&self) -> Option<&str> {
        const DEFINITION_NS: &str = "/definitions/";
        self.fragment.as_ref().and_then(|fragment| {
            fragment.strip_prefix(DEFINITION_NS)
        })
    }
}

#[derive(Clone)]
pub struct SchemaContext<'a> {
    pub schema_store: &'a SchemaStore<'a>,
    pub schema: &'a SchemaObject,
    pub uri: Option<SchemaUri>,
    pub is_uri_root: bool,
}

impl<'a> SchemaContext<'a> {
    pub fn resolve<'b, 'c>(&'b self, schema: &'b SchemaObject) -> SchemaContext<'c>
    where
        'b: 'c,
    {
        if schema.is_ref() {
            let mut reference = schema.reference.as_ref().unwrap().clone();

            // If the reference is a local URI replace with the full URI instead
            if reference.starts_with('#') {
                reference =
                    self.uri.as_ref().unwrap().path.as_ref().unwrap().to_owned() + &reference;
            }

            return SchemaContext {
                schema_store: self.schema_store,
                uri: Some(reference.as_str().into()),
                is_uri_root: true,
                schema: self
                    .schema_store
                    .lookup(&reference.as_str().into())
                    .unwrap()
                    .1,
            };
        }

        SchemaContext {
            schema_store: self.schema_store,
            uri: self.uri.clone(),
            is_uri_root: false,
            schema,
        }
    }
}

pub enum SchemaType {
    Specification,
    Extension(String),
}

pub struct SchemaStore<'a> {
    pub ty: SchemaType,
    folder: PathBuf,
    pub schemas: HashMap<String, RootSchema>,
    base: Option<&'a SchemaStore<'a>>,
}

fn lookup_fragment<'a>(schema: &'a RootSchema, uri: &SchemaUri) -> Option<&'a SchemaObject> {
    if let Some(definition_name) = uri.definition_name() {
        let found_schema = schema.definitions.get(definition_name).unwrap();
        match found_schema {
            Schema::Object(object) => Some(object),
            _ => unreachable!(),
        }
    } else {
        None
    }
}

impl<'a> SchemaStore<'a> {
    pub fn new_specification(folder: &Path) -> Self {
        Self {
            ty: SchemaType::Specification,
            schemas: HashMap::new(),
            folder: PathBuf::from(folder),
            base: None,
        }
    }

    pub fn new_extension(
        folder: &Path,
        specification: &'a SchemaStore,
        extension_name: String,
    ) -> Self {
        Self {
            ty: SchemaType::Extension(extension_name),
            folder: PathBuf::from(folder),
            schemas: HashMap::new(),
            base: Some(specification),
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let dir = match read_dir(&self.folder) {
            Ok(dir) => dir,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        };

        for entry in dir.into_iter().filter_map(Result::ok) {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with("schema.json") {
                self.read(&file_name)?;
            }
        }

        Ok(())
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
        let root_schema =
            serde_json::from_reader(reader).map_err(|e| MyError::FailedToOpenSchema {
                path: full_path.clone(),
                inner: Box::new(e),
            })?;

        self.schemas.insert(id.to_string(), root_schema);
        Ok(())
    }

    pub fn is_local_uri(&self, uri: &SchemaUri) -> bool {
        self.schemas.contains_key(uri.path.as_ref().unwrap())
    }

    pub fn make_context(&self, uri: &SchemaUri) -> SchemaContext {
        let (_, object) = self.lookup(uri).unwrap();
        SchemaContext {
            schema_store: self,
            uri: Some(uri.clone()),
            is_uri_root: true,
            schema: object,
        }
    }

    pub fn lookup(&self, uri: &SchemaUri) -> Option<(&SchemaStore, &SchemaObject)> {
        // Try in base first
        if let Some(base) = self.base {
            if let Some(object) = base.lookup(uri) {
                return Some(object);
            }
        }

        if let Some(root) = self.schemas.get(uri.path.as_ref().unwrap()) {
            if uri.fragment.is_some(){
                lookup_fragment(root, uri).map(|fragment| (self, fragment))
            } else {
                Some((self, &root.schema))
            }
        } else {
            None
        }
    }
}
