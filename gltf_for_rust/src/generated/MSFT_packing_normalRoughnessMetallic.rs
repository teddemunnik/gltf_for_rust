#![allow(clippy::all, unused_imports)]
mod gltf_msft_packing_normal_roughness_metallic {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    mod normal_roughness_metallic_texture {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///A texture with the packing Normal (RG), Roughness (B), Metallic (A).
        pub struct NormalRoughnessMetallicTexture {
            #[serde(default)]
            ///The index of the texture.
            pub index: Option<i64>,
        }
    }
    pub use normal_roughness_metallic_texture::NormalRoughnessMetallicTexture;
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension the specifies a packing of normal, roughness and metallic in a single texture.
    pub struct GltfMsftPackingNormalRoughnessMetallic {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "normalRoughnessMetallicTexture")]
        #[serde(default)]
        ///A texture with the packing Normal (RG), Roughness (B), Metallic (A).
        pub normal_roughness_metallic_texture: Option<NormalRoughnessMetallicTexture>,
    }
    impl crate::GltfExtension for GltfMsftPackingNormalRoughnessMetallic {
        fn extension_name() -> &'static str {
            "MSFT_packing_normalRoughnessMetallic"
        }
    }
    impl crate::GltfObject for GltfMsftPackingNormalRoughnessMetallic {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_msft_packing_normal_roughness_metallic::GltfMsftPackingNormalRoughnessMetallic;
