#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines properties to model physically plausible optical transparency.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default = "get_default_ior")]
            ///The average index of refraction of the material, ignoring differences between frequencies of light. Default of 1.33 is for liquid water.
            pub ior: f64,
            #[serde(rename = "transmissionFactor")]
            #[serde(default = "get_default_transmission_factor")]
            ///The base percentage of non-specularly reflected light that is transmitted through the surface. i.e. of the light that penetrates a surface (isn't specularly reflected), what percentage is transmitted and not diffusely re-emitted from the surface?
            pub transmission_factor: f64,
            #[serde(rename = "transmissionTexture")]
            #[serde(default)]
            ///The percentage of non-specularly reflected light that is transmitted through the surface. i.e. of the light that penetrates a surface (isn't specularly reflected), what percentage is transmitted and not diffusely re-emitted from the surface? This will be multiplied by the transmissionFactor.
            pub transmission_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_ior() -> f64 {
            1.33f64
        }
        fn get_default_transmission_factor() -> f64 {
            1f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "ADOBE_materials_thin_transparency"
        }
    }
}
