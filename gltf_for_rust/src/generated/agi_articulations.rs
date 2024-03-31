#![allow(clippy::all, unused_imports)]
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF Extension for an individual node in a glTF model, to associate it with the model's root AGI_articulations object.
        pub struct Extension {
            #[serde(rename = "articulationName")]
            #[serde(default)]
            ///The name of an Articulation that applies to this node.
            pub articulation_name: Option<String>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(rename = "isAttachPoint")]
            #[serde(default)]
            ///Set to true to indicate that this node's origin and orientation act as an attach point for external objects, analysis, or effects.
            pub is_attach_point: Option<bool>,
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
            "AGI_articulations"
        }
    }
}
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF Extension that defines metadata for applying external analysis or effects to a model.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///An array of articulations.
            pub articulations: Vec<crate::generated::agi_articulations::Articulation>,
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
            "AGI_articulations"
        }
    }
}
mod articulation {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A model articulation definition.
    pub struct Articulation {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "pointingVector")]
        #[serde(default)]
        ///The local forward vector for the associated node, for the purpose of pointing at a target or other object.
        pub pointing_vector: Option<[f64; 3usize]>,
        ///The name of this articulation.  The articulation name must be unique within this model.  Articulation names may not contain spaces.
        pub name: String,
        ///An array of stages, each of which defines a degree of freedom of movement.
        pub stages: Vec<crate::generated::agi_articulations::ArticulationStage>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Articulation {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use articulation::Articulation;
mod articulation_stage {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///One stage of a model articulation definition.
    pub struct ArticulationStage {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "maximumValue")]
        ///The maximum value for the range of motion of this articulation stage.
        pub maximum_value: f64,
        #[serde(rename = "initialValue")]
        ///The initial value for this articulation stage.
        pub initial_value: f64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "minimumValue")]
        ///The minimum value for the range of motion of this articulation stage.
        pub minimum_value: f64,
        #[serde(rename = "type")]
        ///The type of motion applied by this articulation stage.
        pub ty: serde_json::Value,
        ///The name of this articulation stage.  The articulation stage name must be unique only within the containing articulation.  Articulation Stage names may not contain spaces.
        pub name: String,
    }
    impl crate::GltfObject for ArticulationStage {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use articulation_stage::ArticulationStage;
