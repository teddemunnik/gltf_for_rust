#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Deserialize, Serialize};
        use serde_json::{Map, Value};
        mod normal_roughness_metallic_texture {
            use serde::{Deserialize, Serialize};
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
        pub struct Extension {
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
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "MSFT_packing_normalRoughnessMetallic"
        }
    }
}
