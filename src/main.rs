
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


fn read<P: AsRef<Path> + Copy>(path: P) -> Result<RootSchema, MyError> where PathBuf: From<P>{
    let file = File::open(path).map_err(|e| MyError::FailedToOpenSchema(PathBuf::from(path)))?;
    let reader = BufReader::new(file);
    let root_schema = serde_json::from_reader(reader).map_err(|e| MyError::FailedToOpenSchema(PathBuf::from(path)))?;
    Ok(root_schema)
}

struct ObjectReference{}

fn handle_type(schema: &Schema) -> Result<TokenStream, Box<dyn Error>>{
    let schema = match schema{
        Schema::Object(object) => object,
        _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
    };

    // TODO: Handle cross object refs
    if schema.is_ref(){
        return Ok(quote!{ ObjectReference })
    }

    if let Some(subschema) = &schema.subschemas{
        let the_schema = subschema.all_of.as_ref().unwrap().first().unwrap();
        return Ok(quote!{ ObjectReference });
    }


    match &schema.instance_type{
        Some(SingleOrVec::Single(a)) => match **a{
            InstanceType::Null => Err(Box::new(MyError::UnhandledInstanceType(schema.instance_type.clone()))),
            InstanceType::Boolean => Ok(quote!{ bool }),
            InstanceType::Object => Err(Box::new(MyError::UnhandledInstanceType(schema.instance_type.clone()))),
            InstanceType::Array => handle_array(schema),
            InstanceType::Number => Ok(quote!{ f64 }),
            InstanceType::String => Ok(quote!{ String}),
            InstanceType::Integer => Ok(quote!{ i64 }),
        },
        _ => Ok(quote!{serde_json::Value})
    }
}
fn handle_array(schema: &SchemaObject) -> Result<TokenStream, Box<dyn Error>>{
    let array = schema.array.as_ref().unwrap();
    match array.items.as_ref(){
      Some(SingleOrVec::Single(a)) => {
        let inner_type = handle_type(&a)?;
        Ok(quote!{ Vec<#inner_type> })
      },
        _ => Err(Box::new(MyError::UnhandledArrayItemType))
    }
}

fn write_rust(schema: &RootSchema, writer: &mut dyn std::io::Write){
    let metadata = schema.schema.metadata.as_ref().unwrap();
    let comment = metadata.description.as_ref().unwrap();
    let name = Ident::new(metadata.title.as_ref().unwrap(), Span::call_site());

    let mut property_tokens = Vec::new();
    let object_schema = schema.schema.object.as_ref().unwrap();
    for (name, schema) in object_schema.properties.iter(){

        let rusty_name = name.to_case(Case::Snake);


        let typer = handle_type(schema).unwrap();



        // If it's not a required type we wrap with Optional
        /*if !object_schema.required.contains(name){
            typer = quote!{ Option<#typer> };
            additional_attributes.push(quote!{ #[serde(skip_serializing_if="Option::is_none")]});
        }*/

        let mut comment = None;
        if let Schema::Object(object_schema) = schema{

            comment = object_schema.metadata.as_ref().map(|metadata| metadata.description.clone());
        }


        let ident = Ident::new(&rusty_name, Span::call_site());
        let docstring = comment.map(|x| quote!{ #[doc=#x] });
        property_tokens.push(quote!{
            #[serde(rename = #name)]
            #docstring
            #ident: #typer
        })
    }


    write!(writer, "{}", quote!{
        #[doc=#comment]
        struct #name{
            #(#property_tokens),*
        }
    });
}

fn main(){
    // Load the root schema
    let schema = read("vendor/gltf/specification/2.0/schema/gLTF.schema.json").unwrap();

    let mut output = File::create("output_bindings.rs").unwrap();
    let mut writer = BufWriter::new(output);
    write_rust(&schema, &mut writer);
}