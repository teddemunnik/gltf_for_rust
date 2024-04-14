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
            ///Density of the medium given as the average distance that light travels in the medium before interacting with a particle. The value is given in world space. When undefined, the value is assumed to be infinite.
            pub attenuation_distance: Option<f64>,
            #[serde(rename = "thicknessFactor")]
            #[serde(default = "get_default_thickness_factor")]
            ///The thickness of the volume beneath the surface. The value is given in the coordinate space of the mesh. A value greater than 0 turns the mesh into a volume with a homogeneous medium, enabling refraction, absorption and subsurface scattering. The actual value may be ignored by renderers that are able to derive the thickness from the mesh (ray tracer).
            pub thickness_factor: f64,
            #[serde(rename = "thicknessTexture")]
            #[serde(default)]
            ///A texture that defines the thickness of the volume, stored in the G channel. Will be multiplied by thicknessFactor.
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
