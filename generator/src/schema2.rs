use std::collections::BTreeMap;
use std::default::Default;
use std::error::Error;
use std::fs::{File, read_dir};
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::Value;
use thiserror::Error;

use crate::schema_uri::SchemaUri;

#[derive(Serialize, Deserialize, Debug)]
pub struct RootSchema {
    #[serde(flatten)]
    schema: Schema,
}

impl RootSchema {
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

#[derive(Debug, Clone)]
pub struct SchemaContext {
    meta: SchemaStoreMeta,
    uri: SchemaUri,
}

impl SchemaContext {
    pub fn with_subpath(&self, path: String) -> Self {
        let mut uri = self.uri.clone();
        //uri.instance_path.push(path);

        SchemaContext {
            meta: self.meta.clone(),
            uri,
        }
    }

    pub fn uri(&self) -> &SchemaUri {
        &self.uri
    }

    pub fn meta(&self) -> &SchemaStoreMeta {
        &self.meta
    }

    pub fn relative_url(&self) -> &str {
        &self.uri.path
    }

    pub fn is_schema_root(&self) -> bool {
        self.uri.is_schema_root()
    }

    pub fn definition_name(&self) -> Option<&str> {
        self.uri.definition_name()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Metadata {
    #[serde(rename = "$id")]
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "gltf_detailedDescription")]
    pub detailed_description: Option<String>,
    pub default: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ObjectRules {
    properties: BTreeMap<String, Schema>,

    #[serde(default = "Schema::true_schema")]
    additional_properties: Schema,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ArrayRules {
    items: Schema,
    min_items: Option<usize>,
    max_items: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SchemaObject {
    #[serde(rename = "$defs", alias = "definitions")]
    defs: BTreeMap<String, Schema>,

    #[serde(flatten)]
    metadata: Metadata,

    #[serde(default, rename = "type")]
    ty: Option<SingleOrVec<InstanceType>>,

    any_of: Vec<Schema>,

    all_of: Vec<Schema>,

    #[serde(flatten)]
    object_rules: ObjectRules,

    #[serde(flatten)]
    array_rules: ArrayRules,

    #[serde(rename = "const")]
    const_value: Option<Value>,

    #[serde(rename = "enum")]
    enum_values: Option<Vec<Value>>,

    #[serde(rename = "$ref")]
    reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Schema {
    Boolean(bool),
    Object(Box<SchemaObject>),
}

impl Default for Schema {
    fn default() -> Self {
        Schema::true_schema()
    }
}

impl Schema {
    pub fn true_schema() -> Schema {
        Schema::Boolean(true)
    }
    pub fn false_schema() -> Schema {
        Schema::Boolean(false)
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            Schema::Object(object) => object.metadata.title.as_deref(),
            _ => None,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Schema::Object(object) => object.metadata.description.as_deref(),
            serde_json => None,
        }
    }

    pub fn detailed_description(&self) -> Option<&str> {
        match self {
            Schema::Object(object) => object.metadata.detailed_description.as_deref(),
            _ => None
        }
    }

    pub fn default(&self) -> Option<&Value> {
        match self {
            Schema::Object(object) => object.metadata.default.as_ref(),
            _ => None,
        }
    }
    pub fn any_of<'a>(&'a self, context: &'a SchemaContext) -> SubSchemaIterator<'a> {
        match self {
            Schema::Boolean(_) => SubSchemaIterator::empty(context),
            Schema::Object(object) => SubSchemaIterator {
                context,
                inner: object.any_of.iter().enumerate(),
            },
        }
    }

    pub fn all_of<'a>(&'a self, context: &'a SchemaContext) -> SubSchemaIterator<'a> {
        match self {
            Schema::Boolean(_) => SubSchemaIterator::empty(context),
            Schema::Object(object) => SubSchemaIterator {
                context,
                inner: object.all_of.iter().enumerate(),
            },
        }
    }

    pub fn instance_type(&self) -> InstanceTypes {
        InstanceTypes(match self {
            Schema::Object(object) => object.as_ref().ty.as_ref(),
            _ => None
        })
    }

    pub fn properties<'a>(&'a self, context: &'a SchemaContext) -> PropertiesIterator<'a> {
        match self {
            Schema::Boolean(_) => PropertiesIterator::empty(context),
            Schema::Object(object) => PropertiesIterator {
                context,
                inner: object.object_rules.properties.iter(),
            },
        }
    }

    pub fn additional_properties(
        &self,
        context: &SchemaContext,
    ) -> Option<(SchemaContext, &Schema)> {
        match self {
            Schema::Object(object) => Some((
                context.with_subpath("additional_properties".into()),
                &object.object_rules.additional_properties
            )),
            _ => None,
        }
    }

    pub fn required(&self) -> &[String] {
        match self {
            Schema::Object(object) => &object.object_rules.required,
            _ => Default::default(),
        }
    }

    pub fn const_value(&self) -> Option<&Value> {
        match self {
            Schema::Object(object) => object.const_value.as_ref(),
            _ => None,
        }
    }

    pub fn enum_values(&self) -> Option<&[Value]> {
        match self {
            Schema::Object(object) => object.enum_values.as_deref(),
            _ => None,
        }
    }

    pub fn items(&self, context: &SchemaContext) -> Option<(SchemaContext, &Schema)> {
        match self {
            Schema::Object(object) => Some((
                context.with_subpath("items".into()),
                &object.array_rules.items,
            )),
            _ => None,
        }
    }

    pub fn min_items(&self) -> Option<usize> {
        match self {
            Schema::Object(object) => object.array_rules.min_items,
            _ => None,
        }
    }
    pub fn max_items(&self) -> Option<usize> {
        match self {
            Schema::Object(object) => object.array_rules.max_items,
            _ => None,
        }
    }

    pub fn reference(&self) -> Option<&str> {
        match self {
            Schema::Object(object) => object.reference.as_deref(),
            _ => None
        }
    }
}

pub struct SubSchemaIterator<'a> {
    context: &'a SchemaContext,
    inner: std::iter::Enumerate<std::slice::Iter<'a, Schema>>,
}

impl<'a> Iterator for SubSchemaIterator<'a> {
    type Item = (SchemaContext, &'a Schema);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(index, schema)| {
            let mut context = self.context.clone();
            //context.uri.instance_path.push(index.to_string());
            (context, schema)
        })
    }
}

impl<'a> SubSchemaIterator<'a> {
    pub fn empty(context: &SchemaContext) -> SubSchemaIterator {
        SubSchemaIterator {
            inner: Default::default(),
            context,
        }
    }
}

pub struct PropertiesIterator<'a> {
    context: &'a SchemaContext,
    inner: std::collections::btree_map::Iter<'a, String, Schema>,
}

impl<'a> Iterator for PropertiesIterator<'a> {
    type Item = (SchemaContext, &'a str, &'a Schema);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, value)| {
            let mut context = self.context.clone();
            //context.uri.instance_path.push(key.clone());
            (context, key.as_ref(), value)
        })
    }
}

impl<'a> PropertiesIterator<'a> {
    pub fn empty(context: &'a SchemaContext) -> Self {
        Self {
            inner: Default::default(),
            context,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InstanceType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Clone)]
pub enum SchemaStoreMeta {
    Core,
    Extension(String),
}

pub struct SchemaStore {
    meta: SchemaStoreMeta,
    folder: String,
    map: BTreeMap<String, RootSchema>,
}

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Io Error: {0}")]
    IoError(std::io::Error),

    #[error("Failed to open schema {0}: {1}")]
    FailedToOpenSchema(String, Box<dyn Error>),
}

impl SchemaStore {
    pub fn read(meta: SchemaStoreMeta, folder: &str) -> Result<SchemaStore, SchemaError> {
        let dir = match read_dir(&folder) {
            Ok(dir) => dir,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Ok(SchemaStore {
                    meta,
                    folder: folder.to_string(),
                    map: BTreeMap::new(),
                });
            }
            Err(e) => return Err(SchemaError::IoError(e)),
        };

        let mut map = BTreeMap::new();
        for entry in dir.into_iter().filter_map(Result::ok) {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !file_name.ends_with("schema.json") {
                continue;
            }

            let mut full_path = PathBuf::from(folder);
            full_path.push(file_name.clone());

            // Read the requested schema
            let file = File::open(&full_path).map_err(|e| {
                SchemaError::FailedToOpenSchema(
                    full_path.to_string_lossy().to_string(),
                    Box::new(e),
                )
            })?;
            let reader = BufReader::new(file);
            let root_schema = serde_json::from_reader(reader).map_err(|e| {
                SchemaError::FailedToOpenSchema(
                    full_path.to_string_lossy().to_string(),
                    Box::new(e),
                )
            })?;

            map.insert(file_name.clone(), root_schema);
        }

        Ok(SchemaStore {
            meta,
            folder: folder.to_string(),
            map,
        })
    }

    pub fn schemas(&self) -> SchemaIterator {
        SchemaIterator {
            store: self,
            inner: self.map.iter(),
        }
    }

    pub fn is_local_uri(&self, uri: &SchemaUri) -> bool {
        self.map.contains_key(&uri.path)
    }
}

pub struct SchemaResolver<'a> {
    order: Vec<&'a SchemaStore>,
}

impl<'a> SchemaResolver<'a> {
    pub fn extension(specification: &'a SchemaStore, extension: &'a SchemaStore) -> Self {
        SchemaResolver {
            order: vec![extension, specification],
        }
    }

    pub fn specification(specification: &'a SchemaStore) -> Self {
        SchemaResolver {
            order: vec![specification],
        }
    }

    pub fn resolve(&self, uri: &SchemaUri, context: Option<&SchemaUri>) -> Option<(SchemaContext, &Schema)> {
        // No path part means we look up in the context
        let schema_path: &str = if uri.path.is_empty() {
            context.unwrap().path.as_ref()
        } else {
            uri.path.as_ref()
        };

        // Find the schema store containing the URI
        let (store, schema) = match self.order.iter().map(|store| store.map.get(schema_path).map(|schema| (*store, schema))).flatten().next() {
            Some(tuple) => tuple,
            None => return None,
        };

        let schema = match uri.definition_name() {
            Some(def) => match &schema.schema {
                Schema::Object(object) => match object.defs.get(def) {
                    Some(def) => def,
                    _ => return None,
                }
                _ => return None
            },
            _ => &schema.schema
        };

        let context = SchemaContext { meta: store.meta.clone(), uri: uri.clone() };
        Some((context, schema))
    }
}


pub struct SchemaIterator<'a> {
    store: &'a SchemaStore,
    inner: std::collections::btree_map::Iter<'a, String, RootSchema>,
}

impl<'a> Iterator for SchemaIterator<'a> {
    type Item = (SchemaContext, &'a Schema);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(file, schema)| (
            SchemaContext { meta: self.store.meta.clone(), uri: SchemaUri { path: file.clone(), fragment: None } },
            &schema.schema))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SingleOrVec<T> {
    Single(Box<T>),
    Vec(Vec<T>),
}


#[derive(Debug)]
pub struct InstanceTypes<'a>(Option<&'a SingleOrVec<InstanceType>>);

impl<'a> InstanceTypes<'a> {
    pub fn is_only(&self, ty: &InstanceType) -> bool {
        match self.0 {
            Some(SingleOrVec::Single(single)) => single.as_ref().eq(ty),
            Some(SingleOrVec::Vec(v)) if v.len() == 1 => v[0].eq(ty),
            _ => false,
        }
    }

    pub fn single(&self) -> Option<InstanceType> {
        match self.0 {
            Some(SingleOrVec::Single(single)) => Some(**single),
            Some(SingleOrVec::Vec(vec)) if vec.len() == 1 => Some(vec[0]),
            _ => None
        }
    }
}

