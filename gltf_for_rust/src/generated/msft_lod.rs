#![allow(clippy::all, unused_imports)]
mod gltf_msft_lod {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension for specifying levels of detail (LOD).
    pub struct GltfMsftLod {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///Array containing the indices of progressively lower LOD nodes.
        pub ids: Vec<i64>,
    }
    impl crate::GltfExtension for GltfMsftLod {
        fn extension_name() -> &'static str {
            "MSFT_lod"
        }
    }
    impl crate::GltfObject for GltfMsftLod {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_msft_lod::GltfMsftLod;
