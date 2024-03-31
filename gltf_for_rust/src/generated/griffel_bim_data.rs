#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        mod node_property {
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};
            #[derive(Serialize, Deserialize, Debug)]
            ///Key value pair - unique property (instance or type) attached to nodes. Name and value are referenced by index of corresponding root level collection.
            pub struct NodeProperty {
                ///Index of a property name in the root level collection.
                pub name: i64,
                ///Index of a property value in the root level collection.
                pub value: i64,
            }
        }
        pub use node_property::NodeProperty;
        mod node_type {
            use serde::{Serialize, Deserialize};
            use serde_json::{Map, Value};
            #[derive(Serialize, Deserialize, Debug)]
            ///Set of properties which are common for many nodes.
            pub struct NodeType {
                #[serde(default)]
                ///Name of the type.
                pub name: Option<String>,
                ///Collection of indices which point to corresponding properties of the type.
                pub properties: Vec<i64>,
            }
        }
        pub use node_type::NodeType;
        #[derive(Serialize, Deserialize, Debug)]
        ///Domain specific properties for glTF nodes.
        pub struct Extension {
            ///Collection of unique property name - property value pairs.
            pub properties: Vec<NodeProperty>,
            #[serde(rename = "propertyNames")]
            ///Collection of unique property names.
            pub property_names: Vec<String>,
            #[serde(rename = "propertyValues")]
            ///Collection of unique property values.
            pub property_values: Vec<String>,
            #[serde(rename = "types")]
            #[serde(default)]
            ///Collection of types - common sets of properties for many nodes.
            pub tys: Vec<NodeType>,
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "GRIFFEL_bim_data"
        }
    }
}
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///References type and instance properties of the node and/or buffer where those properties can be found by node ID.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "bufferView")]
            #[serde(default)]
            ///Index of the buffer view which points to the buffer with the data for this node.
            pub buffer_view: Option<i64>,
            #[serde(default)]
            ///Collection of indices which point to corresponding instance properties of the node. (Instance properties are unique to the node. They override the same type properties.)
            pub properties: Vec<i64>,
            #[serde(rename = "type")]
            #[serde(default)]
            ///Index of a type in the root level collection. (Type is a set of properties which are common for many nodes.)
            pub ty: Option<i64>,
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
            "GRIFFEL_bim_data"
        }
    }
}
