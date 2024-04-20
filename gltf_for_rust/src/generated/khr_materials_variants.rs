#![allow(clippy::all, unused_imports)]
mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        mod variant {
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};
            #[derive(Serialize, Deserialize, Debug)]
            ///An object defining a valid material variant
            pub struct Variant {
                #[serde(default)]
                ///JSON object with extension-specific objects.
                pub extensions: Option<Map<String, Value>>,
                #[serde(default)]
                ///Application-specific data.
                pub extras: Option<serde_json::Value>,
                ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
                pub name: String,
            }
            impl crate::GltfObject for Variant {
                fn extensions(&self) -> &Option<Map<String, Value>> {
                    &self.extensions
                }
            }
        }
        pub use variant::Variant;
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines a material variations for mesh primitives
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            pub variants: Vec<Variant>,
        }
        impl crate::GltfExtension for Extension {
            fn extension_name() -> &'static str {
                "KHR_materials_variants"
            }
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
}
mod mesh_primitive {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        mod mapping {
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};
            #[derive(Serialize, Deserialize, Debug)]
            pub struct Mapping {
                #[serde(default)]
                ///JSON object with extension-specific objects.
                pub extensions: Option<Map<String, Value>>,
                #[serde(default)]
                ///Application-specific data.
                pub extras: Option<serde_json::Value>,
                ///A reference to the material associated with the given array of variants.
                pub material: i64,
                #[serde(default)]
                ///The optional user-defined name of this variant material mapping.  This is not necessarily unique.
                pub name: Option<String>,
                ///An array of index values that reference variants defined in the glTF root's extension object.
                pub variants: Vec<i64>,
            }
            impl crate::GltfObject for Mapping {
                fn extensions(&self) -> &Option<Map<String, Value>> {
                    &self.extensions
                }
            }
        }
        pub use mapping::Mapping;
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            ///An array of object values that associate an indexed material to a set of variants.
            pub mappings: Vec<Mapping>,
        }
        impl crate::GltfExtension for Extension {
            fn extension_name() -> &'static str {
                "KHR_materials_variants"
            }
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
}
