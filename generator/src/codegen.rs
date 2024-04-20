use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde_json::Value;

use crate::{Enum, GeneratedManifest, naming, ObjectPrototype, plural_to_singular, Property, RustTypeWriter, Type};
use crate::module_builder::ModuleBuilder;
use crate::naming::{generate_enum_type_identifier, generate_option_identifier, generate_property_identifier};
use crate::schema::{SchemaResolver, SchemaStoreMeta};
use crate::schema_uri::SchemaUri;

fn write_embedded_enum(
    property_name: &str,
    enumeration: &Enum,
    default: &Option<Value>,
) -> TokenStream {
    let enum_identifier = generate_enum_type_identifier(property_name);
    let enum_options = enumeration.options.iter().map(|option| {
        let identifier = generate_option_identifier(option);

        let is_default = match &default {
            Some(Value::String(string)) => string == option,
            _ => false,
        };

        let default_declaration = is_default.then(|| quote! { #[default] });
        quote! {
            #[serde(rename=#option)]
            #default_declaration
            #identifier
        }
    });

    let default_declaration = default.as_ref().map(|_| quote! { #[derive(Default)] });
    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #default_declaration
        pub enum #enum_identifier{
            #(#enum_options),*
        }
    }
}

fn write_embedded_type(
    property_name: &str,
    ty: &Type,
    default: &Option<Value>,
    resolver: &SchemaResolver,
) -> anyhow::Result<Option<TokenStream>> {
    Ok(match ty {
        Type::Array(array) => {
            write_embedded_type(property_name, array.item.as_ref(), &None, resolver)?
        }
        Type::EmbeddedObject { name, prototype } => Some(
            generate_structure(
                &name
                    .clone()
                    .unwrap_or_else(|| plural_to_singular(property_name)),
                prototype,
                None,
                resolver,
            )
                .with_context(|| {
                    format!(
                        "Failed to generate embedded type {}",
                        name.as_ref().unwrap()
                    )
                })?,
        ),
        Type::Enum(enumeration) => Some(write_embedded_enum(property_name, enumeration, default)),
        _ => None,
    })
}

fn write_property(
    resolver: &SchemaResolver,
    writer: &mut RustTypeWriter,
    property: &Property,
) -> anyhow::Result<TokenStream> {
    let rust_type = match (&property.ty, property.optional) {
        // Remove the Option for optional Vec's with a minimum length of 1
        // This way we can guarantee this invariant by telling serde to not serialize zero length vecs.
        (Type::Array(array_type), true)
        if array_type.min_length.is_some() && array_type.min_length.unwrap() == 1 =>
            {
                generate_rust_type(resolver, &property.ty, &property.name)
            }

        (_, true) => {
            let rust_type: TokenStream = generate_rust_type(resolver, &property.ty, &property.name);
            quote! { Option::<#rust_type> }
        }
        _ => generate_rust_type(resolver, &property.ty, &property.name),
    };

    let property_identifier = generate_property_identifier(&property.name);
    let property_identifier_name = property_identifier.to_string();

    if let Some(embedded_type) =
        write_embedded_type(&property.name, &property.ty, &property.default, resolver)?
    {
        writer.embedded_types.push(embedded_type);
    }

    let default_declaration = match property.optional {
        true => Some(quote! { #[serde(default)]}),
        false => None,
    };

    // If the property identifier is different from the one in the spec we need to add a serde
    // rename to make it match the spec.
    let rename_declaration =
        if property_identifier_name.partial_cmp(&property.name) != Some(Ordering::Equal) {
            let name = &property.name;
            Some(quote![#[serde(rename = #name)]])
        } else {
            None
        };

    let docstring = property.comment.as_ref().map(|x| quote! { #[doc=#x] });
    Ok(quote! {
        #rename_declaration
        #default_declaration
        #docstring
        pub #property_identifier: #rust_type
    })
}

fn generate_structure(
    name: &str,
    prototype: &ObjectPrototype,
    extension: Option<&str>,
    resolver: &SchemaResolver,
) -> anyhow::Result<TokenStream> {
    let mod_identifier = &naming::generate_property_identifier(name);
    let type_identifier = naming::generate_type_identifier(name);

    let mut property_tokens = Vec::new();
    let mut type_writer = RustTypeWriter::new();
    for property in prototype.properties.iter() {
        property_tokens.push(
            write_property(resolver, &mut type_writer, property)
                .with_context(|| format!("failed to write property {}", property.name))?,
        )
    }

    let doc = prototype
        .comment
        .as_ref()
        .map(|comment| quote! { #[doc=#comment]});
    let embedded_types = &type_writer.embedded_types;
    let default_declarations = &type_writer.default_declarations;

    // Trait implementation if the object is an extension root
    let gltf_extension_trait = extension.map(|extension| {
        quote! {
            impl crate::GltfExtension for #type_identifier{
                fn extension_name() -> &'static str{
                    #extension
                }
            }
        }
    });

    // Trait implementation if the object supports extensions
    let gltf_object_trait = if prototype
        .properties
        .iter()
        .any(|property| property.name.eq("extensions"))
    {
        Some(quote! {
            impl crate::GltfObject for #type_identifier{
                fn extensions(&self) -> &Option<Map<String, Value>>{
                    &self.extensions
                }
            }
        })
    } else {
        None
    };

    Ok(quote! {
        mod #mod_identifier{
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};

            #(#embedded_types)*

            #[derive(Serialize, Deserialize, Debug)]
            #doc
            pub struct #type_identifier{
                #(#property_tokens),*
            }

            #gltf_extension_trait

            #gltf_object_trait

            #(#default_declarations)*

        }
        pub use #mod_identifier::#type_identifier;

    })
}

fn generate_named_type_path(resolver: &SchemaResolver, uri: &SchemaUri) -> TokenStream {
    let (context, schema) = resolver.resolve(uri, None).unwrap();

    let name = naming::get_canonical_name(&context, schema).unwrap();
    let type_name = Ident::new(&name, Span::call_site());

    match context.meta() {
        SchemaStoreMeta::Core => quote! { crate::generated::gltf::#type_name },
        SchemaStoreMeta::Extension(extension) => {
            let ident = naming::generate_base_module_identifier(extension);
            quote! { crate::generated::#ident::#type_name}
        }
    }
}

fn generate_rust_type(resolver: &SchemaResolver, ty: &Type, field_name: &String) -> TokenStream {
    match ty {
        Type::Any => quote! { serde_json::Value },
        Type::Array(array_type) => {
            let item_rust_type = generate_rust_type(resolver, &array_type.item, field_name);
            quote! { Vec::< #item_rust_type > }
        }
        Type::FixedArray(array_type) => {
            let fixed_length = array_type.length as usize;
            let rust_item_type = generate_rust_type(resolver, &array_type.item, field_name);
            quote! { [#rust_item_type; #fixed_length ]}
        }
        Type::Boolean => quote! { bool },
        Type::Integer => quote! { i64 },
        Type::Number => quote! { f64 },
        Type::String => quote! { String },
        Type::Enum(_) => {
            let ident = Ident::new(&field_name.to_case(Case::UpperCamel), Span::call_site());
            quote! { #ident }
        }
        Type::TypedObject(uri) => generate_named_type_path(resolver, uri),
        Type::MapOfObjects => quote! { Map<String, Value> },
        Type::EmbeddedObject { name, prototype: _ } => {
            let ident = Ident::new(
                &name
                    .clone()
                    .unwrap_or_else(|| plural_to_singular(field_name.as_str()))
                    .to_case(Case::UpperCamel),
                Span::call_site(),
            );
            quote! { #ident }
        }
    }
}

pub fn write_module(module: &ModuleBuilder) -> anyhow::Result<()> {
    let types: Vec<TokenStream> = module
        .types
        .values()
        .map(|ty| {
            generate_structure(
                &ty.name,
                &ty.prototype,
                ty.extension.as_deref(),
                module.resolver,
            )
        })
        .collect::<anyhow::Result<Vec<TokenStream>>>()?;

    let rust = quote! {
        #![allow(clippy::all, unused_imports)]

        #(#types)*
    };

    let file: syn::File = syn::parse2(rust).unwrap();
    let output_base = &module.output_base;
    let name = &module.name;
    let output = File::create(format!("{output_base}/{name}.rs")).unwrap();
    let mut writer = BufWriter::new(output);
    write!(writer, "{}", prettyplease::unparse(&file))?;

    Ok(())
}

pub fn write_root_module(generated_path: &str, generated_manifest: &GeneratedManifest) {
    let output = File::create(format!("{generated_path}/mod.rs")).unwrap();
    let mut writer = BufWriter::new(output);

    let extension_modules: Vec<TokenStream> = generated_manifest
        .extension_modules
        .iter()
        .map(|module_name| {
            let ident = Ident::new(module_name, Span::call_site());
            quote! { pub mod #ident; }
        })
        .collect();

    let rust_file: syn::File = syn::parse2(quote! {
        pub mod gltf;
        #(#extension_modules)*
    })
        .unwrap();

    write!(writer, "{}", prettyplease::unparse(&rust_file)).unwrap();
}
