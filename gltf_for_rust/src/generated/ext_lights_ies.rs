#![allow(clippy::all, unused_imports)]
mod gltf_ext_lights_ies {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension that enables the use of IES light profiles.
    pub struct GltfExtLightsIes {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        pub lights: Vec<Map<String, Value>>,
    }
    impl crate::GltfExtension for GltfExtLightsIes {
        fn extension_name() -> &'static str {
            "EXT_lights_ies"
        }
    }
    impl crate::GltfObject for GltfExtLightsIes {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_ext_lights_ies::GltfExtLightsIes;
mod node_ext_lights_ies {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeExtLightsIes {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///RGB value for the light's color in linear space.
        pub color: Option<[f64; 3usize]>,
        ///The id of the light profile referenced by this node.
        pub light: i64,
        #[serde(default)]
        ///Non-negative factor to scale the light's intensity.
        pub multiplier: Option<f64>,
    }
    impl crate::GltfExtension for NodeExtLightsIes {
        fn extension_name() -> &'static str {
            "EXT_lights_ies"
        }
    }
    impl crate::GltfObject for NodeExtLightsIes {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use node_ext_lights_ies::NodeExtLightsIes;
