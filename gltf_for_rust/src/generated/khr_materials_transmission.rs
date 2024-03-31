#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the optical transmission of a material.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "transmissionFactor")]
            #[serde(default = "get_default_transmission_factor")]
            ///The base percentage of light transmitted through the surface.
            pub transmission_factor: f64,
            #[serde(rename = "transmissionTexture")]
            #[serde(default)]
            ///A texture that defines the transmission percentage of the surface, sampled from the R channel. These values are linear, and will be multiplied by transmissionFactor.
            pub transmission_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_transmission_factor() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_transmission"
        }
    }
}
