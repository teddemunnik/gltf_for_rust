#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines anisotropy.
        pub struct Extension {
            #[serde(rename = "anisotropyStrength")]
            #[serde(default = "get_default_anisotropy_strength")]
            ///The anisotropy strength.
            pub anisotropy_strength: f64,
            #[serde(rename = "anisotropyRotation")]
            #[serde(default = "get_default_anisotropy_rotation")]
            ///The rotation of the anisotropy.
            pub anisotropy_rotation: f64,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "anisotropyTexture")]
            #[serde(default)]
            ///The anisotropy texture.
            pub anisotropy_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_anisotropy_strength() -> f64 {
            0f64
        }
        fn get_default_anisotropy_rotation() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_anisotropy"
        }
    }
}
