#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the strength of the specular reflection.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "specularColorFactor")]
            #[serde(default = "get_default_specular_color_factor")]
            ///This is an additional RGB color parameter that tints the specular reflection of non-metallic surfaces. At grazing angles, the reflection still blends to white, and the parameter has not effect on metals. The value is linear.
            pub specular_color_factor: [f64; 3usize],
            #[serde(rename = "specularColorTexture")]
            #[serde(default)]
            ///A texture that defines the specular color in the RGB channels (encoded in sRGB). This will be multiplied by specularColorFactor.
            pub specular_color_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "specularFactor")]
            #[serde(default = "get_default_specular_factor")]
            ///This parameter scales the amount of specular reflection on non-metallic surfaces. It has no effect on metals.
            pub specular_factor: f64,
            #[serde(rename = "specularTexture")]
            #[serde(default)]
            ///A texture that defines the specular factor in the alpha channel. This will be multiplied by specularFactor.
            pub specular_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_specular_color_factor() -> [f64; 3usize] {
            [1f64, 1f64, 1f64]
        }
        fn get_default_specular_factor() -> f64 {
            1f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_specular"
        }
    }
}
