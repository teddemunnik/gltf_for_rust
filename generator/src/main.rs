use schemars::{schema::{SchemaObject, RootSchema, self}, visit::{Visitor, visit_schema_object}};
use std::{fs::File, io::BufWriter, array};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use schemars::visit::visit_root_schema;
use quote::quote;
use std::vec::Vec;
use schemars::schema::{InstanceType, Schema, SingleOrVec};
use proc_macro2::{Ident, Span, TokenStream};
use convert_case::{Case, Casing};
use std::error::Error;
use thiserror::Error;
use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::io::Write;

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to open schema")]
    FailedToOpenSchema(PathBuf),
    #[error("Unhandled instance type")]
    UnhandledInstanceType(Option<SingleOrVec<InstanceType>>),
    #[error("Unhandled array item type")]
    UnhandledArrayItemType
}

struct Enum{
    options: Vec<String>,
}

struct ArrayType{
    min_length: Option<u32>,
    item: Box<Type>
}

struct FixedArrayType{
    item: Box<Type>,
    length: u32,
}

enum Type{
    Any,
    Array(ArrayType),
    FixedArray(FixedArrayType),
    TypedObject(String),
    UntypedObject,
    String,
    Boolean,
    Number,
    Integer,
    Enum(Enum),
    MapOfObjects
}

#[derive(Clone)]
struct SchemaContext<'a>{
    schema_store: &'a SchemaStore,
    path: PathBuf,
    schema: &'a SchemaObject
}

impl<'a> SchemaContext<'a>{
    fn resolve<'b, 'c>(&'b self, schema: &'b SchemaObject) -> SchemaContext<'c> where 'b : 'c{
        if schema.is_ref(){
            let mut path = self.path.clone();
            path.pop();
            let path = path.join(schema.reference.as_ref().unwrap());

            return SchemaContext{
                schema_store: self.schema_store,
                schema: &self.schema_store.schemas.get(&path).unwrap().schema,
                path,
            };
        }

        SchemaContext{ 
            schema_store: self.schema_store,
            path: self.path.clone(),
            schema
        }
    }
}

fn handle_field(schema: &SchemaContext) -> Result<Type, Box<dyn Error>>{
    // If we have an allOf with a single entry we can use it as our type
    if let Some(subschema) = &schema.schema.subschemas{
        if let Some(Schema::Object(single_all_of)) = subschema.all_of.as_ref().and_then(|all_of| all_of.first()){
            let the_schema = schema.resolve(single_all_of);
            if let Some(_) = the_schema.schema.object{
                if let Some(id) = the_schema.schema.metadata.as_ref().and_then(|md| md.id.as_ref() ){
                    return Ok(Type::TypedObject(id.clone()));
                }
            }
        }
    }

    if let Some(enumeration) = try_match_enum(schema){
        return Ok(Type::Enum(enumeration));
    }

    handle_type(schema)
}

fn try_match_enum(schema: &SchemaContext) -> Option<Enum>{
    let any_of = match schema.schema.subschemas.as_ref().and_then(|subschema| subschema.any_of.as_ref()){
        Some(any_of) => any_of,
        _ => return None,
    };

    let mut options = Vec::new();
    for option in any_of{
        let option = match option{
            Schema::Object(object) => object,
            _ => return None,
        };

        let is_string_constant = match option.const_value.as_ref(){
            Some(Value::String(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_string = match option.instance_type.as_ref(){
            Some(SingleOrVec::Single(single)) => {
                match single.as_ref(){
                    InstanceType::String => true,
                    _ => false
                }
            },
            _ => false
        };

        if !is_string && !is_string_constant{
            return None;
        }
    }

    Some(Enum { options })
}

fn handle_type(schema: &SchemaContext) -> Result<Type, Box<dyn Error>>{
    // Try to match based on an instance type if one exists
    let ty = match &schema.schema.instance_type{
        Some(SingleOrVec::Single(a)) => match **a{
            InstanceType::Null => Err(Box::new(MyError::UnhandledInstanceType(schema.schema.instance_type.clone())) as Box<dyn Error>),
            InstanceType::Boolean => Ok(Some(Type::Boolean)),
            InstanceType::Object => {
                // An object with no properties, but only additionalProperties, as a typed map
                if let Some(additional_properties) = schema.schema.object.as_ref().unwrap().additional_properties.as_ref(){
                    if schema.schema.object.as_ref().unwrap().properties.is_empty(){
                        Ok(Some(Type::MapOfObjects))
                    }
                    else{
                        unreachable!();
                    }

                // If the object has a title, it's a typed object
                } else if let Some(id) = schema.schema.metadata.as_ref().and_then(|metadata| metadata.id.as_ref()){
                    Ok(Some(Type::TypedObject(id.clone())))
                } else{
                    Ok(Some(Type::UntypedObject))
                }
            }
            InstanceType::Array => Ok(Some(handle_array(schema)?)),
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
    if let Some(Schema::Object(single_all_of)) = schema.schema.subschemas.as_ref().and_then(|schema| schema.all_of.as_ref()).and_then(|all_of| all_of.first()){
        let single_all_of = schema.resolve(single_all_of);
        return handle_type(&single_all_of);
    }

    Ok(Type::Any)
}

fn handle_array(schema: &SchemaContext) -> Result<Type, Box<dyn Error>>{
    let array = schema.schema.array.as_ref().unwrap();

    let single_item_type = match array.items.as_ref(){
        Some(SingleOrVec::Single(ty)) => match ty.as_ref(){
            Schema::Object(object) => object,
            _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
        },
        _ => return Err(Box::new(MyError::UnhandledArrayItemType)),
    };

    let single_item_type = schema.resolve(single_item_type);

    let item_type = handle_type(&single_item_type)?;

    if array.min_items.is_some() && array.min_items == array.max_items{
        let fixed_length = array.min_items.unwrap();
        return Ok(Type::FixedArray(FixedArrayType { item: Box::new(item_type), length: fixed_length }));
    }

    Ok(Type::Array(ArrayType { min_length: array.min_items, item: Box::new(item_type) }))
}

fn generate_rust_type(schema_lookup: &HashMap<String, SchemaContext>, ty: &Type, field_name: &String) -> TokenStream{
    match ty{
        Type::Any => quote!{ serde_json::Value },
        Type::Array(array_type) => { let item_rust_type = generate_rust_type(schema_lookup, &array_type.item, field_name); return quote!{ Vec::< #item_rust_type > }; },
        Type::FixedArray(array_type) => { 
            let fixed_length = array_type.length as usize;
            let rust_item_type = generate_rust_type(schema_lookup, &array_type.item,field_name);
            quote!{ [#rust_item_type; #fixed_length ]}
        },
        Type::Boolean => quote!{ bool },
        Type::Integer => quote!{ i64 },
        Type::Number => quote!{ f64 },
        Type::String => quote!{ String },
        Type::Enum(enumeration) => { let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site()); quote!{ #ident } },
        Type::TypedObject(id) => { 
            let name = schema_lookup.get(id).unwrap().schema.metadata.as_ref().unwrap().title.as_ref().unwrap().replace(" ", "");
            let module = Ident::new(&name.to_lowercase(), Span::call_site());
            let ident = Ident::new(&name, Span::call_site());
             quote!{ super::#module::#ident } 
        },
        Type::UntypedObject => quote!{ Map<String, Value> },
        Type::MapOfObjects => quote!{ Map<String, Value> }
    }
}

fn schedule_types(open_types: &mut Vec<String>, closed_types: &HashSet<String>, ty: &Type){
    match ty{
        Type::Array(array_type) => schedule_types(open_types, closed_types, array_type.item.as_ref()),
        Type::TypedObject(id) => {
            if !closed_types.contains(id) && !open_types.contains(id){
                open_types.push(id.clone());
            }
        },
        _ => (),
    }
}

struct Property{
    ty: Type,
    optional: bool,
    default: Option<Value>,
    comment: Option<String>,
}

fn recursive_read_properties(properties: &mut HashMap<String, Property>, schema: &SchemaContext){
    // First read properties from 'base' schemas
    let base_schema = schema.schema.subschemas.as_ref().and_then(|subschema| subschema.all_of.as_ref()).and_then(|all_of| all_of.first());
    if let Some(Schema::Object(base)) = base_schema{
        let base = schema.resolve(base);
        recursive_read_properties(properties, &base);
    }

    // Then add our own properties
    let object_schema = schema.schema.object.as_ref().unwrap();
    for (name, field_schema) in object_schema.properties.iter(){
        let field_schema = match field_schema{
            Schema::Object(object) => object,
            _ => unreachable!(),
        };
        let field_schema = schema.resolve(field_schema);

        let property = properties.entry(name.clone()).or_insert(Property{ ty: Type::Any, optional: true, comment: None, default: None});

        match property.ty{
            Type::Any => property.ty = handle_field(&field_schema).unwrap(),
            _ => (),
        }

        if property.comment.is_none(){
            property.comment = field_schema.schema.metadata.as_ref().and_then(|metadata| metadata.description.clone());
        }

        if property.default.is_none(){
            property.default = field_schema.schema.metadata.as_ref().and_then(|metadata| metadata.default.clone());
        }
        
        if object_schema.required.contains(name){
            property.optional = false;
        }
    }

}

fn write_rust(schema_lookup: &HashMap<String, SchemaContext>, schema: &SchemaContext, writer: &mut dyn std::io::Write, open_types: &mut Vec<String>, closed_types: &HashSet<String>){
    let metadata = schema.schema.metadata.as_ref().unwrap();
    let comment = metadata.description.as_ref();
    let name = Ident::new(&metadata.title.as_ref().unwrap().replace(" ", ""), Span::call_site());

    let mut properties = HashMap::new();
    recursive_read_properties(&mut properties, &schema);

    let mut embedded_enums = Vec::new();


    let mut property_tokens = Vec::new();
    for (name, property) in properties.iter(){
        let rusty_name = name.to_case(Case::Snake).replace("type", "ty");
        schedule_types(open_types, closed_types, &property.ty);

        

        

        let rust_type = match (&property.ty , property.optional){
            // Remove the Option for optional Vec's with a minimum length of 1
            // This way we can guarantee this invariant by telling serde to not serialize zero length vecs.
            (Type::Array(array_type), true) if array_type.min_length.is_some() && array_type.min_length.unwrap() == 1 => generate_rust_type(schema_lookup, &property.ty, name),

            // Remove the Option for optional properties which have a default value specified.
            (_, true) if property.default.is_none() => { let rust_type : TokenStream = generate_rust_type(schema_lookup,&property.ty, name); quote!{ Option::<#rust_type> } },
            _ => generate_rust_type(schema_lookup,&property.ty, name),
        };

        // Embedded enums
        if let Type::Enum(enumeration) = &property.ty{
            let rusty_enum_name = Ident::new(&name.to_case(Case::UpperCamel), Span::call_site());
            let enum_options = enumeration.options.iter().map(|option|{
                let identifier = Ident::new(&option.to_case(Case::UpperCamel).replace(&['/'], ""), Span::call_site());

                let is_default = match &property.default{
                    Some(Value::String(string)) => string == option,
                    _ => false
                };

                let default_declaration = is_default.then(|| quote!{ #[default] } );
                quote!{
                    #[serde(rename=#option)]
                    #default_declaration
                    #identifier
                }
            });

            let default_declaration = property.default.as_ref().map(|_| quote!{ #[derive(Default)] });
            embedded_enums.push(quote!{
                #[derive(Serialize, Deserialize)]
                #[serde(untagged)]
                #default_declaration
                enum #rusty_enum_name{
                    #(#enum_options),*
                }
            });
        }

        let default_is_type_default = match property.default.as_ref(){
            Some(default) => match property.ty{
                Type::Enum(_) => true,
                Type::Boolean => !default.as_bool().unwrap(),
                Type::Integer => default.as_i64().unwrap() == 0,
                Type::Number => default.as_f64().unwrap() == 0.0,
                _ => false,
            },
            _ => false
        };

        let default_declaration = default_is_type_default.then(|| quote!{ #[serde(default)]} );


        let ident = Ident::new(&rusty_name, Span::call_site());
        let docstring = property.comment.as_ref().map(|x| quote!{ #[doc=#x] });
        property_tokens.push(quote!{
            #[serde(rename = #name)]
            #default_declaration
            #docstring
            #ident: #rust_type
        })
    }

    let doc = match comment{
        Some(comment) => Some(quote!{ #[doc=#comment]}),
        _ => None,
    };

    let mod_name = Ident::new(&metadata.title.as_ref().unwrap().replace(" ", "").to_lowercase(), Span::call_site());

    write!(writer, "{}", quote!{
        mod #mod_name{
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};

            #(#embedded_enums)*

            #[derive(Serialize, Deserialize)]
            #doc
            pub struct #name{
                #(#property_tokens),*
            }

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
        let file = File::open(path.clone()).map_err(|_| MyError::FailedToOpenSchema(PathBuf::from(path.clone())))?;
        let reader = BufReader::new(file);
        let mut root_schema = serde_json::from_reader(reader).map_err(|_| MyError::FailedToOpenSchema(PathBuf::from(path.clone())))?;

        // Read any requested subschema 
        let mut current_directory = PathBuf::from(path.clone());
        current_directory.pop();
        let mut visitor = ReferencedSchemaVisitor{ store: self, current_directory, result: Result::Ok(())};
        visit_root_schema(&mut visitor, &mut root_schema);
        self.schemas.insert(path.into(), root_schema);
        Ok(())
    }

}

fn main(){
    let mut schema_store = SchemaStore{ schemas: HashMap::new(), roots: Vec::new() };
    // Load the root schema
    schema_store.read_root("vendor\\gltf\\specification\\2.0\\schema\\glTF.schema.json").unwrap();

    let output = File::create("gltf_for_rust\\src\\lib.rs").unwrap();
    let mut writer = BufWriter::new(output);

    // Build a map to lookup named types in the schemas
    let mut schema_lookup = HashMap::new();
    for (path, schema) in schema_store.schemas.iter(){

        if let Some(id) = schema.schema.metadata.as_ref().and_then(|metadata| metadata.id.as_ref()){
            schema_lookup.insert(id.clone(), SchemaContext{ schema_store: &schema_store, path: path.clone(), schema: &schema.schema });
        }
    }

    // Collect root types:
    let mut closed_types = HashSet::new();
    let mut open_types = Vec::new();
    for path in schema_store.roots.iter(){
        let schema = schema_store.schemas.get(path).unwrap();
        if let Some(id) = schema.schema.metadata.as_ref().and_then(|metadata| metadata.id.as_ref()){
            open_types.push(id.clone());
        }
    }

    while !open_types.is_empty(){
        let id = open_types.pop().unwrap();
        closed_types.insert(id.clone());
        let schema = *schema_lookup.get(&id).as_ref().unwrap();

        write_rust(&schema_lookup, schema, &mut writer, &mut open_types, &closed_types);
    }
}