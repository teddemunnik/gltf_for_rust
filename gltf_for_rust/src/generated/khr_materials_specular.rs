#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the strength of the specular reflection.
        pub struct Extension {
            #[serde(rename = "specularColorTexture")]
            #[serde(default)]
            ///A texture that defines the F0 color of the specular reflection.
            pub specular_color_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "specularFactor")]
            #[serde(default = "get_default_specular_factor")]
            ///The strength of the specular reflection.
            pub specular_factor: f64,
            #[serde(rename = "specularTexture")]
            #[serde(default)]
            ///A texture that defines the specular factor in the alpha channel.
            pub specular_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "specularColorFactor")]
            #[serde(default = "get_default_specular_color_factor")]
            ///The F0 RGB color of the specular reflection.
            pub specular_color_factor: [f64; 3usize],
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_specular_factor() -> f64 {
            1f64
        }
        fn get_default_specular_color_factor() -> [f64; 3usize] {
            [1f64, 1f64, 1f64]
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_specular"
        }
    }
}
