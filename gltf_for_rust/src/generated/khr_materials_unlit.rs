#![allow(clippy::all, unused_imports)]
mod material_khr_materials_unlit {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that defines the unlit material model.
    pub struct MaterialKhrMaterialsUnlit {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
    }
    impl crate::GltfExtension for MaterialKhrMaterialsUnlit {
        fn extension_name() -> &'static str {
            "KHR_materials_unlit"
        }
    }
    impl crate::GltfObject for MaterialKhrMaterialsUnlit {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_khr_materials_unlit::MaterialKhrMaterialsUnlit;
