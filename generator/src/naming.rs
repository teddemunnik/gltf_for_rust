//! Utilities for extracting consistent names out of the gltf specification

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};

use crate::schema::{Schema, SchemaContext};

/// Creates a canonical name of a schema object
/// The canonical name will use UpperCamelCase formatting
pub fn get_canonical_name(context: &SchemaContext, schema: &Schema) -> Option<String> {
    // Use the definition name
    //if context.is_root() {
    //    if let Some(definition_name) = context.definition_name(){
    //        return Some(definition_name.to_lowercase().to_case(Case::UpperCamel));
    //    }
    //}

    // Get the schema name
    let uri = context.uri();
    if uri.is_schema_root() {
        if let Some(no_suffix) = uri.path.strip_suffix(".schema.json") {
            return Some(
                no_suffix
                    .replace("glTF", "gltf")
                    .replace('.', " ")
                    .to_case(Case::UpperCamel),
            );
        }
    }

    // Or use the title
    if let Some(title) = schema.title() {
        let title = title.to_lowercase().to_case(Case::UpperCamel);

        // Remove module prefix from title
        let prefix_end = title.find(' ');
        if let Some(prefix_end) = prefix_end {
            if title[0..prefix_end].starts_with("khr_") {
                return Some(title[(prefix_end + 1)..].to_string());
            }
        }
        Some(title.clone())
    } else {
        None
    }
}

pub fn generate_property_identifier(name: &str) -> Ident {
    // Replace keywords
    let name = match name.to_lowercase().as_str() {
        "type" => "ty",
        _ => name,
    };

    // Remove unsupported characters
    let name = name.replace('@', "");

    // Convert to the field snake case
    let name = name.to_case(Case::Snake);

    Ident::new(&name, Span::call_site())
}

pub fn generate_option_identifier(name: &str) -> Ident {
    Ident::new(
        &name.replace(['/', '.'], " ").to_case(Case::UpperCamel),
        Span::call_site(),
    )
}

pub fn generate_base_module_identifier(name: &str) -> Ident {
    Ident::new(
        &name
            .replace('.', " ")
            .replace("glTF", "gltf")
            .to_case(Case::Snake),
        Span::call_site(),
    )
}

pub fn generate_enum_type_identifier(name: &str) -> Ident {
    Ident::new(&name.to_case(Case::UpperCamel), Span::call_site())
}

pub fn generate_type_identifier(name: &str) -> Ident {
    Ident::new(&name.to_case(Case::UpperCamel), Span::call_site())
}
