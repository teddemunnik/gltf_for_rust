#![allow(clippy::all, unused_imports)]
mod gltf_msft_packing_occlusion_roughness_metallic {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    mod normal_texture {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///A texture which contains two channel (RG) normal map.
        pub struct NormalTexture {
            #[serde(default)]
            ///The index of the texture.
            pub index: Option<i64>,
        }
    }
    pub use normal_texture::NormalTexture;
    mod occlusion_roughness_metallic_texture {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///A texture with packing Occlusion (R), Roughness (G), Metallic (B).
        pub struct OcclusionRoughnessMetallicTexture {
            #[serde(default)]
            ///The index of the texture.
            pub index: Option<i64>,
        }
    }
    pub use occlusion_roughness_metallic_texture::OcclusionRoughnessMetallicTexture;
    mod roughness_metallic_occlusion_texture {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///A texture with packing Roughness (R), Metallic (G), Occlusion (B).
        pub struct RoughnessMetallicOcclusionTexture {
            #[serde(default)]
            ///The index of the texture.
            pub index: Option<i64>,
        }
    }
    pub use roughness_metallic_occlusion_texture::RoughnessMetallicOcclusionTexture;
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension the specifies a packing of occlusion, roughness and metallic in a single texture and a two channel normal map.
    pub struct GltfMsftPackingOcclusionRoughnessMetallic {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "normalTexture")]
        #[serde(default)]
        ///A texture which contains two channel (RG) normal map.
        pub normal_texture: Option<NormalTexture>,
        #[serde(rename = "occlusionRoughnessMetallicTexture")]
        #[serde(default)]
        ///A texture with packing Occlusion (R), Roughness (G), Metallic (B).
        pub occlusion_roughness_metallic_texture: Option<
            OcclusionRoughnessMetallicTexture,
        >,
        #[serde(rename = "roughnessMetallicOcclusionTexture")]
        #[serde(default)]
        ///A texture with packing Roughness (R), Metallic (G), Occlusion (B).
        pub roughness_metallic_occlusion_texture: Option<
            RoughnessMetallicOcclusionTexture,
        >,
    }
    impl crate::GltfExtension for GltfMsftPackingOcclusionRoughnessMetallic {
        fn extension_name() -> &'static str {
            "MSFT_packing_occlusionRoughnessMetallic"
        }
    }
    impl crate::GltfObject for GltfMsftPackingOcclusionRoughnessMetallic {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_msft_packing_occlusion_roughness_metallic::GltfMsftPackingOcclusionRoughnessMetallic;
