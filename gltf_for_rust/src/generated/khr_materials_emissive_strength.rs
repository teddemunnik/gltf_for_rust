#![allow(clippy::all, unused_imports)]
mod material_khr_materials_emissive_strength {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that adjusts the strength of emissive material properties.
    pub struct MaterialKhrMaterialsEmissiveStrength {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "emissiveStrength")]
        #[serde(default)]
        ///The strength adjustment to be multiplied with the material's emissive value.
        pub emissive_strength: Option<f64>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsEmissiveStrength {
        fn extension_name() -> &'static str {
            "KHR_materials_emissive_strength"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsEmissiveStrength {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_emissive_strength::MaterialKhrMaterialsEmissiveStrength;
