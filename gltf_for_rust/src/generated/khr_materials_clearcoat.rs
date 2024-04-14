#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the clearcoat material layer.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "clearcoatFactor")]
            #[serde(default = "get_default_clearcoat_factor")]
            ///The clearcoat layer intensity (aka opacity) of the material. A value of 0.0 means the material has no clearcoat layer enabled.
            pub clearcoat_factor: f64,
            #[serde(rename = "clearcoatNormalTexture")]
            #[serde(default)]
            ///A tangent space normal map for the clearcoat layer.  If desired, this may be a reference to the same normal map used by the base material.  If not supplied, no normal mapping is applied to the clear coat layer.
            pub clearcoat_normal_texture: Option<
                crate::generated::gltf::MaterialNormalTextureInfo,
            >,
            #[serde(rename = "clearcoatRoughnessFactor")]
            #[serde(default = "get_default_clearcoat_roughness_factor")]
            ///The clearcoat layer roughness of the material.
            pub clearcoat_roughness_factor: f64,
            #[serde(rename = "clearcoatRoughnessTexture")]
            #[serde(default)]
            ///The clearcoat layer roughness texture. These values are sampled from the G channel.  The values are linear.  Use value 1.0 if no texture is supplied.
            pub clearcoat_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "clearcoatTexture")]
            #[serde(default)]
            ///The clearcoat layer intensity texture. These values are sampled from the R channel.  The values are linear.  Use value 1.0 if no texture is supplied.
            pub clearcoat_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_clearcoat_factor() -> f64 {
            0f64
        }
        fn get_default_clearcoat_roughness_factor() -> f64 {
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
