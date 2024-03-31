#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the clearcoat material layer.
        pub struct Extension {
            #[serde(rename = "clearcoatRoughnessTexture")]
            #[serde(default)]
            ///The clearcoat layer roughness texture.
            pub clearcoat_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "clearcoatRoughnessFactor")]
            #[serde(default = "get_default_clearcoat_roughness_factor")]
            ///The clearcoat layer roughness.
            pub clearcoat_roughness_factor: f64,
            #[serde(rename = "clearcoatTexture")]
            #[serde(default)]
            ///The clearcoat layer intensity texture.
            pub clearcoat_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(rename = "clearcoatFactor")]
            #[serde(default = "get_default_clearcoat_factor")]
            ///The clearcoat layer intensity.
            pub clearcoat_factor: f64,
            #[serde(rename = "clearcoatNormalTexture")]
            #[serde(default)]
            ///The clearcoat normal map texture.
            pub clearcoat_normal_texture: Option<
                crate::generated::gltf::MaterialNormalTextureInfo,
            >,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_clearcoat_roughness_factor() -> f64 {
            0f64
        }
        fn get_default_clearcoat_factor() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_clearcoat"
        }
    }
}
