use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use anyhow::Context;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;

use crate::naming::{
    generate_enum_type_identifier, generate_option_identifier, generate_property_identifier,
};
use crate::schema::{SchemaResolver, SchemaStore};
use crate::schema_uri::SchemaUri;
use crate::{
    generate_rust_type, naming, plural_to_singular, read_typed_object, Enum, ObjectPrototype,
    Property, RustTypeWriter, Type,
};

pub struct TypeDescription {
    pub schema: SchemaUri,
    pub name_override: Option<String>,
    pub extension: Option<String>,
}

pub struct ResolvedType {
    name: String,
    prototype: ObjectPrototype,
    extension: Option<String>,
}

pub struct ModuleBuilder<'a> {
    output_base: String,
    name: String,
    types: HashMap<SchemaUri, ResolvedType>,
    open_list: Vec<TypeDescription>,
    store: &'a SchemaStore,
    resolver: &'a SchemaResolver<'a>,
}

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

impl<'a> ModuleBuilder<'a> {
    pub fn new(
        output_base: &str,
        name: &str,
        resolver: &'a SchemaResolver,
        store: &'a SchemaStore,
    ) -> Self {
        Self {
            output_base: String::from(output_base),
            name: String::from(name),
            types: HashMap::new(),
            resolver,
            store,
            open_list: Vec::new(),
        }
    }
    pub fn push(&mut self, ty: TypeDescription) {
        self.open_list.push(ty);
    }

    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::TypedObject(schema) => {
                if self.types.contains_key(schema)
                    || self.open_list.iter().any(|item| item.schema.eq(schema))
                {
                    return;
                }

                self.open_list.push(TypeDescription {
                    schema: schema.clone(),
                    name_override: None,
                    extension: None,
                })
            }
            Type::Array(array) => self.visit_type(&array.item),
            _ => (),
        }
    }

    pub fn traverse(&mut self) {
        while let Some(ty) = self.open_list.pop() {
            if !self.store.is_local_uri(&ty.schema) {
                continue;
            }

            let (context, schema) = self.resolver.resolve(&ty.schema, None).unwrap();

            // Create the object prototype
            let object_type = read_typed_object(self.resolver, &context, schema);

            // Schedule nested types for generation
            for property in object_type.prototype.properties.iter() {
                self.visit_type(&property.ty);
            }

            self.types.insert(
                ty.schema.clone(),
                ResolvedType {
                    name: ty.name_override.unwrap_or(object_type.name),
                    prototype: object_type.prototype,
                    extension: ty.extension,
                },
            );
        }
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        let types: Vec<TokenStream> = self
            .types
            .values()
            .map(|ty| {
                generate_structure(
                    &ty.name,
                    &ty.prototype,
                    ty.extension.as_deref(),
                    self.resolver,
                )
            })
            .collect::<anyhow::Result<Vec<TokenStream>>>()?;

        let rust = quote! {
            #![allow(clippy::all, unused_imports)]

            #(#types)*
        };

        let file: syn::File = syn::parse2(rust).unwrap();
        let output_base = &self.output_base;
        let name = &self.name;
        let output = File::create(format!("{output_base}/{name}.rs")).unwrap();
        let mut writer = BufWriter::new(output);
        write!(writer, "{}", prettyplease::unparse(&file))?;

        Ok(())
    }
}
