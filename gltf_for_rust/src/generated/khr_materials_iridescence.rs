#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines an iridescence effect.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "iridescenceFactor")]
            #[serde(default = "get_default_iridescence_factor")]
            ///The iridescence intensity factor.
            pub iridescence_factor: f64,
            #[serde(rename = "iridescenceIor")]
            #[serde(default = "get_default_iridescence_ior")]
            ///The index of refraction of the dielectric thin-film layer.
            pub iridescence_ior: f64,
            #[serde(rename = "iridescenceTexture")]
            #[serde(default)]
            ///The iridescence intensity texture. The values are sampled from the R channel. These values are linear. If a texture is not given, a value of `1.0` **MUST** be assumed. If other channels are present (GBA), they are ignored for iridescence intensity calculations.
            pub iridescence_texture: Option<crate::generated::gltf::TextureInfo>,
            #[serde(rename = "iridescenceThicknessMaximum")]
            #[serde(default = "get_default_iridescence_thickness_maximum")]
            ///The maximum thickness of the thin-film layer given in nanometers. The value **MUST** be greater than or equal to the value of `iridescenceThicknessMinimum`.
            pub iridescence_thickness_maximum: f64,
            #[serde(rename = "iridescenceThicknessMinimum")]
            #[serde(default = "get_default_iridescence_thickness_minimum")]
            ///The minimum thickness of the thin-film layer given in nanometers. The value **MUST** be less than or equal to the value of `iridescenceThicknessMaximum`.
            pub iridescence_thickness_minimum: f64,
            #[serde(rename = "iridescenceThicknessTexture")]
            #[serde(default)]
            ///The thickness texture of the thin-film layer to linearly interpolate between the minimum and maximum thickness given by the corresponding properties, where a sampled value of `0.0` represents the minimum thickness and a sampled value of `1.0` represents the maximum thickness. The values are sampled from the G channel. These values are linear. If a texture is not given, the maximum thickness **MUST** be assumed. If other channels are present (RBA), they are ignored for thickness calculations.
            pub iridescence_thickness_texture: Option<
                crate::generated::gltf::TextureInfo,
            >,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_iridescence_factor() -> f64 {
            0f64
        }
        fn get_default_iridescence_ior() -> f64 {
            1.3f64
        }
        fn get_default_iridescence_thickness_maximum() -> f64 {
            400f64
        }
        fn get_default_iridescence_thickness_minimum() -> f64 {
            100f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_materials_iridescence"
        }
    }
}
