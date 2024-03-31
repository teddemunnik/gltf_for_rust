#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension defines instance attributes for a node with a mesh.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///A dictionary object, where each key corresponds to instance attribute and each value is the index of the accessor containing attribute's data. Attributes TRANSLATION, ROTATION, SCALE define instance transformation. For "TRANSLATION" the values are FLOAT_VEC3's specifying translation along the x, y, and z axes. For "ROTATION" the values are VEC4's specifying rotation as a quaternion in the order (x, y, z, w), where w is the scalar, with component type `FLOAT` or normalized integer. For "SCALE" the values are FLOAT_VEC3's specifying scaling factors along the x, y, and z axes.
            pub attributes: Option<Map<String, Value>>,
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
            "EXT_mesh_gpu_instancing"
        }
    }
}
