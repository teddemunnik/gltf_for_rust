#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that adjusts the strength of emissive material properties.
        pub struct Extension {
            #[serde(rename = "emissiveStrength")]
            #[serde(default = "get_default_emissive_strength")]
            ///The strength adjustment to be multiplied with the material's emissive value.
            pub emissive_strength: f64,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_emissive_strength() -> f64 {
            1f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_emissive_strength"
        }
    }
}
