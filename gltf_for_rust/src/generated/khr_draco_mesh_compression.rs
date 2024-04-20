#![allow(clippy::all, unused_imports)]
mod mesh_primitive_khr_draco_mesh_compression {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MeshPrimitiveKhrDracoMeshCompression {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///A dictionary object, where each key corresponds to an attribute and its unique attribute id stored in the compressed geometry.
        pub attributes: Map<String, Value>,
        #[serde(rename = "bufferView")]
        ///The index of the bufferView.
        pub buffer_view: i64,
    }
    impl crate::GltfExtension for MeshPrimitiveKhrDracoMeshCompression {
        fn extension_name() -> &'static str {
            "KHR_draco_mesh_compression"
        }
    }
    impl crate::GltfObject for MeshPrimitiveKhrDracoMeshCompression {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use mesh_primitive_khr_draco_mesh_compression::MeshPrimitiveKhrDracoMeshCompression;
