#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines anisotropy.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "anisotropyRotation")]
            #[serde(default = "get_default_anisotropy_rotation")]
            ///The rotation of the anisotropy in tangent, bitangent space, measured in radians counter-clockwise from the tangent. When anisotropyTexture is present, anisotropyRotation provides additional rotation to the vectors in the texture.
            pub anisotropy_rotation: f64,
            #[serde(rename = "anisotropyStrength")]
            #[serde(default = "get_default_anisotropy_strength")]
            ///The anisotropy strength. When anisotropyTexture is present, this value is multiplied by the blue channel.
            pub anisotropy_strength: f64,
            #[serde(rename = "anisotropyTexture")]
            #[serde(default)]
            ///The anisotropy texture. Red and green channels represent the anisotropy direction in [-1, 1] tangent, bitangent space, to be rotated by anisotropyRotation. The blue channel contains strength as [0, 1] to be multiplied by anisotropyStrength.
            pub anisotropy_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_anisotropy_rotation() -> f64 {
            0f64
        }
        fn get_default_anisotropy_strength() -> f64 {
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
