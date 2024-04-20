#![allow(clippy::all, unused_imports)]
mod node_khr_lights_punctual {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeKhrLightsPunctual {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The id of the light referenced by this node.
        pub light: i64,
    }
    impl crate::GltfExtension for NodeKhrLightsPunctual {
        fn extension_name() -> &'static str {
            "KHR_lights_punctual"
        }
    }
    impl crate::GltfObject for NodeKhrLightsPunctual {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use node_khr_lights_punctual::NodeKhrLightsPunctual;
mod gltf_khr_lights_punctual {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct GltfKhrLightsPunctual {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        pub lights: Vec<Map<String, Value>>,
    }
    impl crate::GltfExtension for GltfKhrLightsPunctual {
        fn extension_name() -> &'static str {
            "KHR_lights_punctual"
        }
    }
    impl crate::GltfObject for GltfKhrLightsPunctual {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_khr_lights_punctual::GltfKhrLightsPunctual;
