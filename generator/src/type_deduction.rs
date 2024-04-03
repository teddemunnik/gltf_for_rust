use std::collections::HashSet;

use anyhow::Context;
use schemars::schema::{InstanceType, Schema, SingleOrVec};
use serde_json::Value;

use crate::{
    ArrayType, Enum, FixedArrayType, naming,
    ObjectPrototype, PropertyListBuilder, Type,
};
use anyhow::Context;
use std::collections::HashSet;

use schemars::schema::{InstanceType, Schema, SingleOrVec};
use serde_json::Value;

use crate::schema::{SchemaContext, SchemaUri};
use crate::{naming, ArrayType, Enum, FixedArrayType, ObjectPrototype, PropertyListBuilder, Type};

pub fn handle_field(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<Type> {
    // Extensible string enum
    if let Some(enumeration) = try_match_string_enum(schema) {
        return Ok(Type::Enum(enumeration));
    }

    // Extensible int enum
    if try_match_int_enum(schema).is_some() {
        return Ok(Type::Integer);
    }

    // Specific string enum
    if let Some(enumeration) = schema.schema.enum_values.as_ref() {
        return Ok(Type::Enum(Enum {
            options: enumeration
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect(),
        }));
    }

    handle_type(schema, open_types, closed_types)
}

fn try_match_string_enum(schema: &SchemaContext) -> Option<Enum> {
    let any_of = match schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|sub_schema| sub_schema.any_of.as_ref())
    {
        Some(any_of) => any_of,
        _ => return None,
    };

    let mut options = Vec::new();
    for option in any_of {
        let option = match option {
            Schema::Object(object) => object,
            _ => return None,
        };

        let is_string_constant = match option.const_value.as_ref() {
            Some(Value::String(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_string = match option.instance_type.as_ref() {
            Some(SingleOrVec::Single(single)) => matches!(single.as_ref(), InstanceType::String),
            _ => false,
        };

        if !is_string && !is_string_constant {
            return None;
        }
    }

    Some(Enum { options })
}

fn try_match_int_enum(schema: &SchemaContext) -> Option<()> {
    let any_of = match schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|sub_schema| sub_schema.any_of.as_ref())
    {
        Some(any_of) => any_of,
        _ => return None,
    };

    let mut options = Vec::new();
    for option in any_of {
        let option = match option {
            Schema::Object(object) => object,
            _ => return None,
        };

        let is_number_constant = match option.const_value.as_ref() {
            Some(Value::Number(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_number = match option.instance_type.as_ref() {
            Some(SingleOrVec::Single(single)) => matches!(single.as_ref(), InstanceType::Integer),
            _ => false,
        };

        if !is_number && !is_number_constant {
            return None;
        }
    }

    Some(())
}

fn handle_object_type(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<Option<Type>> {
    // An object with no properties, but only additionalProperties, as a typed map
    let object_validation = schema.schema.object.as_ref().unwrap();

    if object_validation.additional_properties.as_ref().is_some()
        && schema.schema.object.as_ref().unwrap().properties.is_empty()
    {
        return Ok(Some(Type::MapOfObjects));
    }

    if schema.is_uri_root {
        let uri = schema.uri.as_ref().unwrap();
        return Ok(Some(Type::TypedObject(uri.clone())));
    }

    // Embedded object
    let comment = schema
        .schema
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.description.clone());
    let mut properties = PropertyListBuilder::new();
    properties
        .recursive_read_properties(&schema, open_types, closed_types)
        .context("Failed to read properties for embedded object")?;

    let name = schema
        .uri
        .as_ref()
        .and_then(|uri| naming::get_canonical_name(schema));
    Ok(Some(Type::EmbeddedObject {
        name,
        prototype: ObjectPrototype {
            properties: properties.properties,
            comment,
        },
    }))
}
fn handle_type_from_instance_type(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<Option<Type>> {
    // Try to match based on an instance type if one exists
    match &schema.schema.instance_type {
        Some(SingleOrVec::Single(a)) => match **a {
            InstanceType::Null => {
                return Err(anyhow::anyhow!(
                    "Unhandled instance type {:?}",
                    &schema.schema.instance_type
                ))
            }
            InstanceType::Boolean => Ok(Some(Type::Boolean)),
            InstanceType::Object => handle_object_type(schema, open_types, closed_types),
            InstanceType::Array => Ok(Some(handle_array(schema, open_types, closed_types)?)),
            InstanceType::Number => Ok(Some(Type::Number)),
            InstanceType::String => Ok(Some(Type::String)),
            InstanceType::Integer => Ok(Some(Type::Integer)),
        },
        _ => Ok(None),
    }
}
fn handle_type(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<Type> {
    if let Some(ty) = handle_type_from_instance_type(schema, open_types, closed_types)? {
        return Ok(ty);
    }
    // If there is an allOf with a single entry try to match based of this instead
    if let Some(Schema::Object(single_all_of)) = schema
        .schema
        .subschemas
        .as_ref()
        .and_then(|schema| schema.all_of.as_ref())
        .and_then(|all_of| all_of.first())
    {
        let single_all_of = schema.resolve(single_all_of);
        return handle_type(&single_all_of, open_types, closed_types);
    }

    // Fallback to an any
    Ok(Type::Any)
}

fn handle_array(
    schema: &SchemaContext,
    open_types: &mut Vec<SchemaUri>,
    closed_types: &HashSet<SchemaUri>,
) -> anyhow::Result<Type> {
    let array = schema.schema.array.as_ref().unwrap();

    let single_item_type = match array.items.as_ref() {
        Some(SingleOrVec::Single(ty)) => match ty.as_ref() {
            Schema::Object(object) => object,
            _ => anyhow::bail!("Unhandled array item type {:?}", &array.items),
        },
        _ => anyhow::bail!("Unhandled array item type {:?}", &array.items),
    };

    let single_item_type = schema.resolve(single_item_type);

    let item_type = handle_type(&single_item_type, open_types, closed_types)?;

    if array.min_items.is_some() && array.min_items == array.max_items {
        let fixed_length = array.min_items.unwrap();
        return Ok(Type::FixedArray(FixedArrayType {
            item: Box::new(item_type),
            length: fixed_length,
        }));
    }

    Ok(Type::Array(ArrayType {
        min_length: array.min_items,
        item: Box::new(item_type),
    }))
}
