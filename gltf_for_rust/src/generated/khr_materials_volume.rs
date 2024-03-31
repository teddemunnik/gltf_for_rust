#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the parameters for the volume of a material.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "attenuationColor")]
            #[serde(default = "get_default_attenuation_color")]
            ///Color that white light turns into due to absorption when reaching the attenuation distance.
            pub attenuation_color: [f64; 3usize],
            #[serde(rename = "attenuationDistance")]
            #[serde(default)]
            ///Average distance that light travels in the medium before interacting with a particle.
            pub attenuation_distance: Option<f64>,
            #[serde(rename = "thicknessFactor")]
            #[serde(default = "get_default_thickness_factor")]
            ///Thickness of the volume.
            pub thickness_factor: f64,
            #[serde(rename = "thicknessTexture")]
            #[serde(default)]
            ///Texture that defines the thickness of the volume, stored in the G channel.
            pub thickness_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_attenuation_color() -> [f64; 3usize] {
            [1f64, 1f64, 1f64]
        }
        fn get_default_thickness_factor() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_volume"
        }
    }
}
