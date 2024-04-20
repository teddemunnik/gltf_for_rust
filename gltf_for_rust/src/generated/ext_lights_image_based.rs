#![allow(clippy::all, unused_imports)]
mod gltf_ext_lights_image_based {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct GltfExtLightsImageBased {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        pub lights: Vec<Map<String, Value>>,
    }
    impl crate::GltfExtension for GltfExtLightsImageBased {
        fn extension_name() -> &'static str {
            "EXT_lights_image_based"
        }
    }
    impl crate::GltfObject for GltfExtLightsImageBased {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_ext_lights_image_based::GltfExtLightsImageBased;
mod scene_ext_lights_image_based {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SceneExtLightsImageBased {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The id of the light referenced by this scene.
        pub light: i64,
    }
    impl crate::GltfExtension for SceneExtLightsImageBased {
        fn extension_name() -> &'static str {
            "EXT_lights_image_based"
        }
    }
    impl crate::GltfObject for SceneExtLightsImageBased {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use scene_ext_lights_image_based::SceneExtLightsImageBased;
