
use schemars::{schema::{SchemaObject, RootSchema}, visit::{Visitor, visit_schema_object}};
use std::{fs::File, io::BufWriter};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use schemars::visit::visit_root_schema;
use quote::quote;
use std::vec::Vec;
use schemars::_private::NoSerialize;
use schemars::schema::{InstanceType, Schema, SingleOrVec};
use proc_macro2::{Ident, Span, Literal, TokenStream};
use convert_case::{Case, Casing};
use std::fmt::Display;
use std::error::Error;
use thiserror::Error;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to open schema")]
    FailedToOpenSchema(PathBuf),
    #[error("Failed to resolve schema reference")]
    FailedToResolveReference(PathBuf),
    #[error("Unhandled instance type")]
    UnhandledInstanceType(Option<SingleOrVec<InstanceType>>),
    #[error("Unhandled array item type")]
    UnhandledArrayItemType
}


enum Type{
    Any,
    Array(Box<Type>),
    TypedObject(String),
    UntypedObject,
    String,
    Boolean,
    Number,
    Integer,
}


struct ObjectReference{}

fn handle_field(schema_store: &SchemaStore, current_path: &PathBuf, schema: &SchemaObject) -> Result<Type, Box<dyn Error>>{
    // If we have an allOf with a single entry we can use it as our type
    if let Some(subschema) = &schema.subschemas{
        let the_schema = subschema.all_of.as_ref().unwrap().first().unwrap();
        let the_schema = match the_schema{
            Schema::Object(object) => object,
            _ => unreachable!(),
        };
        let the_schema = schema_store.resolve(current_path, the_schema);
        if let Some(_) = the_schema.object{
            if let Some(name) = the_schema.metadata.as_ref().and_then(|md| md.title.as_ref() ){
                return Ok(Type::TypedObject(name.clone()));
            }
        }
    }

    handle_type(schema_store, current_path, schema)
}

fn handle_type(schema_store: &SchemaStore, current_path: &PathBuf, schema: &SchemaObject) -> Result<Type, Box<dyn Error>>{
    let schema = schema_store.resolve(current_path, schema);

    // Try to match based on an instance type if one exists
    let ty = match &schema.instance_type{
        Some(SingleOrVec::Single(a)) => match **a{
            InstanceType::Null => Err(Box::new(MyError::UnhandledInstanceType(schema.instance_type.clone())) as Box<dyn Error>),
            InstanceType::Boolean => Ok(Some(Type::Boolean)),
            InstanceType::Object => Err(Box::new(MyError::UnhandledInstanceType(schema.instance_type.clone())) as Box<dyn Error>),
            InstanceType::Array => handle_array(schema_store, current_path, schema).map(|ok| Some(ok)),
            InstanceType::Number => Ok(Some(Type::Number)),
            InstanceType::String => Ok(Some(Type::String)),
            InstanceType::Integer => Ok(Some(Type::Integer)),
        },
        _ => Ok(None),
    }?;

    match ty{
        Some(ty) => return Ok(ty),
        _ => ()
    };

    // If there is an allOf with a single entry try to match based of this instead
    if let Some(Schema::Object(single_all_of)) = schema.subschemas.as_ref().and_then(|schema| schema.all_of.as_ref()).and_then(|all_of| all_of.first()){
        return handle_type(schema_store, current_path, single_all_of);
    }

    Ok(Type::Any)
}

fn handle_array(schema_store: &SchemaStore, current_path: &PathBuf, schema: &SchemaObject) -> Result<Type, Box<dyn Error>>{
    let array = schema.array.as_ref().unwrap();
    match array.items.as_ref(){
      Some(SingleOrVec::Single(a)) => {
        match a.as_ref(){
            Schema::Object(object) => {
                let object = schema_store.resolve(current_path, object);
                if let Some(name) = object.metadata.as_ref().and_then(|md| md.title.as_ref()){
                    Ok(Type::Array(Box::new(Type::TypedObject(name.clone().replace(" ", "")))))
                }else{
                    Ok(Type::Array(Box::new(handle_type(schema_store, current_path, object)?)))
                }
            },
            _ => Err(Box::new(MyError::UnhandledArrayItemType)),
        }
      },
        _ => Err(Box::new(MyError::UnhandledArrayItemType))
    }
}

fn generate_rust_type(ty: &Type) -> TokenStream{
    match ty{
        Type::Any => quote!{ serde_json::Value },
        Type::Array(item_type) => { let item_rust_type = generate_rust_type(item_type); return quote!{ Vec::< #item_rust_type > }; },
        Type::Boolean => quote!{ bool },
        Type::Integer => quote!{ i64 },
        Type::Number => quote!{ f64 },
        Type::String => quote!{ String },
        Type::TypedObject(name) => { let ident = Ident::new(name, Span::call_site()); quote!{ #ident } },
        Type::UntypedObject => quote!{ HashMap::<String, serde_json::Value> }
    }
}

fn write_rust(schema_store: &SchemaStore, schema_path: &PathBuf, writer: &mut dyn std::io::Write){
    let schema = schema_store.schemas.get(schema_path).unwrap();

    let metadata = schema.schema.metadata.as_ref().unwrap();
    let comment = metadata.description.as_ref().unwrap();
    let name = Ident::new(metadata.title.as_ref().unwrap(), Span::call_site());

    let mut property_tokens = Vec::new();
    let object_schema = schema.schema.object.as_ref().unwrap();
    for (name, schema) in object_schema.properties.iter(){
        let schema = match schema{
            Schema::Object(object) => object,
            _ => unreachable!(),
        };
        let schema = schema_store.resolve(schema_path, schema);

        let rusty_name = name.to_case(Case::Snake);


        let ty = handle_field(schema_store, schema_path, schema).unwrap();
    
        let optional = !object_schema.required.contains(name);

        let rust_type = match (&ty , optional){
            (Type::Array(_), true) => generate_rust_type(&ty),
            (_, true) => { let rust_type : TokenStream = generate_rust_type(&ty); quote!{ Option::<#rust_type> } },
            _ => generate_rust_type(&ty),
        };

        let comment = schema.metadata.as_ref().map(|metadata| metadata.description.clone());

        let ident = Ident::new(&rusty_name, Span::call_site());
        let docstring = comment.map(|x| quote!{ #[doc=#x] });
        property_tokens.push(quote!{
            #[serde(rename = #name)]
            #docstring
            #ident: #rust_type
        })
    }


    write!(writer, "{}", quote!{
        #[doc=#comment]
        struct #name{
            #(#property_tokens),*
        }
    });
}

struct ReferencedSchemaVisitor<'a>{
    store: &'a mut SchemaStore,
    current_directory: PathBuf,
    result: Result<(), Box<dyn Error>>
}

impl<'a> Visitor for ReferencedSchemaVisitor<'a>{
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        // Don't load more references once one fails
        if self.result.is_err(){
            return;
        }

        // Resolve the schema by path
        if schema.is_ref(){
            let schema_path = self.current_directory.join(schema.reference.as_ref().unwrap());
            self.result = self.store.read(schema_path);
            return;
        }

        visit_schema_object(self, schema)
    }
}

struct SchemaStore{
    schemas: HashMap<PathBuf, RootSchema>,
    roots: Vec<PathBuf>
}

impl SchemaStore{
    fn read_root<P: AsRef<Path>+Clone>(&mut self, path: P) -> Result<(), Box<dyn Error>> where PathBuf: From<P>{
        let result = self.read(path.clone());
        self.roots.push(path.into());
        result
    }

    fn read<P: AsRef<Path>+Clone>(&mut self, path: P) -> Result<(), Box<dyn Error>> where PathBuf: From<P>{
        // Read the requested schema
        let file = File::open(path.clone()).map_err(|e| MyError::FailedToOpenSchema(PathBuf::from(path.clone())))?;
        let reader = BufReader::new(file);
        let mut root_schema = serde_json::from_reader(reader).map_err(|e| MyError::FailedToOpenSchema(PathBuf::from(path.clone())))?;

        // Read any requested subschema 
        let mut current_directory = PathBuf::from(path.clone());
        current_directory.pop();
        let mut visitor = ReferencedSchemaVisitor{ store: self, current_directory, result: Result::Ok(())};
        visit_root_schema(&mut visitor, &mut root_schema);
        self.schemas.insert(path.into(), root_schema);
        Ok(())
    }

    fn resolve<'a, 'b>(&'a self, current_path: &PathBuf, schema: &'a SchemaObject) -> &'b SchemaObject where 'a : 'b{
        if schema.is_ref(){
            let mut path = current_path.clone();
            path.pop();
            let path = path.join(schema.reference.as_ref().unwrap());
            return &self.schemas.get(&path).unwrap().schema;
        }

        schema
    }
}

fn main(){
    let mut schema_store = SchemaStore{ schemas: HashMap::new(), roots: Vec::new() };
    // Load the root schema
    schema_store.read_root("vendor\\gltf\\specification\\2.0\\schema\\glTF.schema.json").unwrap();

    let mut output = File::create("output_bindings.rs").unwrap();
    let mut writer = BufWriter::new(output);

    for root in &schema_store.roots{
        write_rust(&schema_store, root, &mut writer);

    }
}