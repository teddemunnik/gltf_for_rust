#![allow(clippy::all, unused_imports)]
mod material_khr_materials_dispersion {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines the strength of dispersion.
    pub struct MaterialKhrMaterialsDispersion {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///This parameter defines dispersion in terms of the 20/Abbe number formulation.
        pub dispersion: Option<f64>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsDispersion {
        fn extension_name() -> &'static str {
            "KHR_materials_dispersion"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsDispersion {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_dispersion::MaterialKhrMaterialsDispersion;
