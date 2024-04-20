#![allow(clippy::all, unused_imports)]
mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the index of refraction of a material.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///The index of refraction (IOR) is a measured physical number usually in the range between 1 and 2 that determines how much the path of light is bent, or refracted, when entering a material. It also influences the ratio between reflected and transmitted light, calculated from the Fresnel equations.
            pub ior: Option<f64>,
        }
        impl crate::GltfExtension for Extension {
            fn extension_name() -> &'static str {
                "KHR_materials_ior"
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
