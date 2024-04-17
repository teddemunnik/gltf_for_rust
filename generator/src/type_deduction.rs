use anyhow::Context;
use itertools::Itertools;
use serde::de::{EnumAccess, SeqAccess, Visitor};
use serde::Deserializer;
use serde_json::Value;

use crate::{ArrayType, Enum, FixedArrayType, ObjectPrototype, PropertyListBuilder, Type};
use crate::schema::{InstanceType, Schema, SchemaContext, SchemaResolver};
use crate::schema_uri::SchemaUri;

pub fn handle_field(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> anyhow::Result<Type> {
    // Extensible string enum
    if let Some(enumeration) = try_match_string_enum(context, schema) {
        return Ok(Type::Enum(enumeration));
    }

    // Extensible int enum
    if try_match_int_enum(context, schema).is_some() {
        return Ok(Type::Integer);
    }

    // Specific string enum
    if let Some(enumeration) = schema.enum_values() {
        return Ok(Type::Enum(Enum {
            options: enumeration
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect(),
        }));
    }

    handle_type(resolver, context, schema)
}

fn try_match_string_enum(context: &SchemaContext, schema: &Schema) -> Option<Enum> {
    let mut options = Vec::new();
    for (context, option) in schema.any_of(context) {
        let is_string_constant = match option.const_value() {
            Some(Value::String(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_string = option.instance_type().is_only(&InstanceType::String);
        if !is_string && !is_string_constant {
            return None;
        }
    }

    if options.is_empty() {
        return None;
    }

    Some(Enum { options })
}

fn try_match_int_enum(context: &SchemaContext, schema: &Schema) -> Option<()> {
    let mut options = Vec::new();
    for (context, option) in schema.any_of(context) {
        let is_number_constant = match option.const_value() {
            Some(Value::Number(option)) => {
                options.push(option.clone());
                true
            }
            _ => false,
        };

        let is_number = option.instance_type().is_only(&InstanceType::Integer);

        if !is_number && !is_number_constant {
            return None;
        }
    }

    if options.is_empty() {
        return None;
    }

    Some(())
}

fn handle_object_type(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> anyhow::Result<Option<Type>> {
    if schema.additional_properties(context).is_some() && schema.properties(context).count() == 0 {
        return Ok(Some(Type::MapOfObjects));
    }

    if context.is_schema_root() {
        return Ok(Some(Type::TypedObject(context.uri().clone())));
    }

    // Embedded object
    let comment = schema.description().map(|desc| desc.to_string());
    let mut properties = PropertyListBuilder::new();
    properties
        .recursive_read_properties(resolver, &context, schema)
        .context("Failed to read properties for embedded object")?;

    let name = None; // TODO;
    Ok(Some(Type::EmbeddedObject {
        name,
        prototype: ObjectPrototype {
            properties: properties.properties,
            comment,
        },
    }))
}

fn handle_type_from_instance_type(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> anyhow::Result<Option<Type>> {
    // Try to match based on an instance type if one exists
    let single_instance_type = match schema.instance_type().single() {
        Some(ty) => ty,
        None => return Ok(None),
    };

    match single_instance_type {
        InstanceType::Null => anyhow::bail!("Unhandled instance type {:?}", single_instance_type),
        InstanceType::Boolean => Ok(Some(Type::Boolean)),
        InstanceType::Object => handle_object_type(resolver, context, schema),
        InstanceType::Array => Ok(Some(handle_array(
            resolver,
            context,
            schema,
        )?)),
        InstanceType::Number => Ok(Some(Type::Number)),
        InstanceType::String => Ok(Some(Type::String)),
        InstanceType::Integer => Ok(Some(Type::Integer)),
    }
}

fn handle_type(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> anyhow::Result<Type> {
    if let Some(ty) = handle_type_from_instance_type(resolver, context, schema)? {
        return Ok(ty);
    }

    // If there is an allOf with a single entry try to match based of this instead
    if let Ok((context, schema)) = schema.all_of(context).exactly_one() {
        return handle_type(resolver, &context, schema);
    }

    // Check if we deduce from a reference
    if let Some(reference) = schema.reference() {
        let uri = SchemaUri::from_str(reference);
        let (context, reference) = resolver.resolve(&uri, Some(context.uri())).unwrap();
        return handle_type(resolver, &context, reference);
    }

    // Fallback to an any
    Ok(Type::Any)
}

fn handle_array(
    resolver: &SchemaResolver,
    context: &SchemaContext,
    schema: &Schema,
) -> anyhow::Result<Type> {
    let (context, items) = schema.items(context).unwrap();

    let item_type = handle_type(resolver, &context, items)?;

    let min_items = schema.min_items();
    let max_items = schema.max_items();
    if min_items.is_some() && min_items == max_items {
        let fixed_length = min_items.unwrap();
        return Ok(Type::FixedArray(FixedArrayType {
            item: Box::new(item_type),
            length: fixed_length as u32,
        }));
    }

    Ok(Type::Array(ArrayType {
        min_length: min_items,
        item: Box::new(item_type),
    }))
}


