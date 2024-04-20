#![allow(clippy::all, unused_imports)]
mod mesh_ext_mesh_manifold {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension defines manifoldness for a mesh.
    pub struct MeshExtMeshManifold {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "manifoldPrimitive")]
        pub manifold_primitive: crate::generated::gltf::MeshPrimitive,
        #[serde(rename = "mergeIndices")]
        #[serde(default)]
        ///The index of the accessor that contains the vertex sparse indices for merging into a manifold.
        pub merge_indices: Option<i64>,
        #[serde(rename = "mergeValues")]
        #[serde(default)]
        ///The index of the accessor that contains the vertex sparse values for merging into a manifold.
        pub merge_values: Option<i64>,
    }
    impl crate::GltfExtension for MeshExtMeshManifold {
        fn extension_name() -> &'static str {
            "EXT_mesh_manifold"
        }
    }
    impl crate::GltfObject for MeshExtMeshManifold {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use mesh_ext_mesh_manifold::MeshExtMeshManifold;
