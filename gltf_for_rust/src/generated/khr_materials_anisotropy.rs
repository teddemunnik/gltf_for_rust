#![allow(clippy::all, unused_imports)]
mod material_khr_materials_anisotropy {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines anisotropy.
    pub struct MaterialKhrMaterialsAnisotropy {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "anisotropyRotation")]
        #[serde(default)]
        ///The rotation of the anisotropy in tangent, bitangent space, measured in radians counter-clockwise from the tangent. When anisotropyTexture is present, anisotropyRotation provides additional rotation to the vectors in the texture.
        pub anisotropy_rotation: Option<f64>,
        #[serde(rename = "anisotropyStrength")]
        #[serde(default)]
        ///The anisotropy strength. When anisotropyTexture is present, this value is multiplied by the blue channel.
        pub anisotropy_strength: Option<f64>,
        #[serde(rename = "anisotropyTexture")]
        #[serde(default)]
        ///The anisotropy texture. Red and green channels represent the anisotropy direction in [-1, 1] tangent, bitangent space, to be rotated by anisotropyRotation. The blue channel contains strength as [0, 1] to be multiplied by anisotropyStrength.
        pub anisotropy_texture: Option<crate::generated::gltf::TextureInfo>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsAnisotropy {
        fn extension_name() -> &'static str {
            "KHR_materials_anisotropy"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsAnisotropy {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_anisotropy::MaterialKhrMaterialsAnisotropy;
