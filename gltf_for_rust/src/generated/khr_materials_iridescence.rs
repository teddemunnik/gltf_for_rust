#![allow(clippy::all, unused_imports)]
mod material_khr_materials_iridescence {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines an iridescence effect.
    pub struct MaterialKhrMaterialsIridescence {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "iridescenceFactor")]
        #[serde(default)]
        ///The iridescence intensity factor.
        pub iridescence_factor: Option<f64>,
        #[serde(rename = "iridescenceIor")]
        #[serde(default)]
        ///The index of refraction of the dielectric thin-film layer.
        pub iridescence_ior: Option<f64>,
        #[serde(rename = "iridescenceTexture")]
        #[serde(default)]
        ///The iridescence intensity texture. The values are sampled from the R channel. These values are linear. If a texture is not given, a value of `1.0` **MUST** be assumed. If other channels are present (GBA), they are ignored for iridescence intensity calculations.
        pub iridescence_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "iridescenceThicknessMaximum")]
        #[serde(default)]
        ///The maximum thickness of the thin-film layer given in nanometers. The value **MUST** be greater than or equal to the value of `iridescenceThicknessMinimum`.
        pub iridescence_thickness_maximum: Option<f64>,
        #[serde(rename = "iridescenceThicknessMinimum")]
        #[serde(default)]
        ///The minimum thickness of the thin-film layer given in nanometers. The value **MUST** be less than or equal to the value of `iridescenceThicknessMaximum`.
        pub iridescence_thickness_minimum: Option<f64>,
        #[serde(rename = "iridescenceThicknessTexture")]
        #[serde(default)]
        ///The thickness texture of the thin-film layer to linearly interpolate between the minimum and maximum thickness given by the corresponding properties, where a sampled value of `0.0` represents the minimum thickness and a sampled value of `1.0` represents the maximum thickness. The values are sampled from the G channel. These values are linear. If a texture is not given, the maximum thickness **MUST** be assumed. If other channels are present (RBA), they are ignored for thickness calculations.
        pub iridescence_thickness_texture: Option<crate::generated::gltf::TextureInfo>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsIridescence {
        fn extension_name() -> &'static str {
            "KHR_materials_iridescence"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsIridescence {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_iridescence::MaterialKhrMaterialsIridescence;
