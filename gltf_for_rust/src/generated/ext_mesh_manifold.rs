#![allow(clippy::all, unused_imports)]
pub mod mesh {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension defines manifoldness for a mesh.
        pub struct Extension {
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
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "EXT_mesh_manifold"
        }
    }
}
