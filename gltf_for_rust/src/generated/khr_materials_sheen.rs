#![allow(clippy::all, unused_imports)]
mod material_khr_materials_sheen {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines the sheen material model.
    pub struct MaterialKhrMaterialsSheen {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "sheenColorFactor")]
        #[serde(default)]
        ///Color of the sheen layer (in linear space).
        pub sheen_color_factor: Option<[f64; 3usize]>,
        #[serde(rename = "sheenColorTexture")]
        #[serde(default)]
        ///The sheen color (RGB) texture. Stored in channel RGB, the sheen color is in sRGB transfer function.
        pub sheen_color_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "sheenRoughnessFactor")]
        #[serde(default)]
        ///The sheen layer roughness of the material.
        pub sheen_roughness_factor: Option<f64>,
        #[serde(rename = "sheenRoughnessTexture")]
        #[serde(default)]
        ///The sheen roughness (Alpha) texture. Stored in alpha channel, the roughness value is in linear space.
        pub sheen_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsSheen {
        fn extension_name() -> &'static str {
            "KHR_materials_sheen"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsSheen {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_sheen::MaterialKhrMaterialsSheen;
