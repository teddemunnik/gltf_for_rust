#![allow(clippy::all, unused_imports)]
mod material_khr_materials_transmission {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines the optical transmission of a material.
    pub struct MaterialKhrMaterialsTransmission {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "transmissionFactor")]
        #[serde(default)]
        ///The base percentage of non-specularly reflected light that is transmitted through the surface. i.e. of the light that penetrates a surface (isn't specularly reflected), this is the percentage that is transmitted and not diffusely re-emitted.
        pub transmission_factor: Option<f64>,
        #[serde(rename = "transmissionTexture")]
        #[serde(default)]
        ///A texture that defines the transmission percentage of the surface, sampled from the R channel. These values are linear, and will be multiplied by transmissionFactor. This indicates the percentage of non-specularly reflected light that is transmitted through the surface. i.e. of the light that penetrates a surface (isn't specularly reflected), this is the percentage is transmitted and not diffusely re-emitted.
        pub transmission_texture: Option<crate::generated::gltf::TextureInfo>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsTransmission {
        fn extension_name() -> &'static str {
            "KHR_materials_transmission"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsTransmission {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_transmission::MaterialKhrMaterialsTransmission;
