#![allow(clippy::all, unused_imports)]
mod primitive_cesium_primitive_outline {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension for indicating that some edges of a primitive's triangles should be outlined.
    pub struct PrimitiveCesiumPrimitiveOutline {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the accessor providing the list of highlighted lines at the edge of this primitive's triangles.
        pub indices: Option<i64>,
    }
    impl crate::GltfExtension for PrimitiveCesiumPrimitiveOutline {
        fn extension_name() -> &'static str {
            "CESIUM_primitive_outline"
        }
    }
    impl crate::GltfObject for PrimitiveCesiumPrimitiveOutline {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use primitive_cesium_primitive_outline::PrimitiveCesiumPrimitiveOutline;
