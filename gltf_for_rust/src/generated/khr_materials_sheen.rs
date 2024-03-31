#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the sheen material model.
        pub struct Extension {
            #[serde(rename = "sheenRoughnessTexture")]
            #[serde(default)]
            ///The sheen roughness (Alpha) texture.
            pub sheen_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "sheenColorFactor")]
            #[serde(default = "get_default_sheen_color_factor")]
            ///Color of the sheen layer (in linear space).
            pub sheen_color_factor: [f64; 3usize],
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(rename = "sheenColorTexture")]
            #[serde(default)]
            ///The sheen color (RGB) texture.
            pub sheen_color_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "sheenRoughnessFactor")]
            #[serde(default = "get_default_sheen_roughness_factor")]
            ///The sheen layer roughness.
            pub sheen_roughness_factor: f64,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_sheen_color_factor() -> [f64; 3usize] {
            [0f64, 0f64, 0f64]
        }
        fn get_default_sheen_roughness_factor() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_sheen"
        }
    }
}
