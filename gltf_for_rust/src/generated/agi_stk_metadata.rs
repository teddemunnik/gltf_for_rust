#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF Extension that defines metadata for use with STK (Systems Tool Kit).
        pub struct Extension {
            #[serde(rename = "solarPanelGroups")]
            #[serde(default)]
            ///An array of solar panel groups.
            pub solar_panel_groups: Vec<
                crate::generated::agi_stk_metadata::SolarPanelGroup,
            >,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
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
            "AGI_stk_metadata"
        }
    }
}
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF Extension for an individual node in a glTF model, to associate it with the model's root AGI_stk_metadata object.
        pub struct Extension {
            #[serde(rename = "noObscuration")]
            #[serde(default)]
            ///Set to true to indicate that this node's geometry does not obscure any sensors' view in the STK Sensor Obscuration tool.
            pub no_obscuration: Option<bool>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "solarPanelGroupName")]
            #[serde(default)]
            ///The name of a Solar Panel Group that includes this node.
            pub solar_panel_group_name: Option<String>,
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
            "AGI_stk_metadata"
        }
    }
}
mod solar_panel_group {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A solar panel group definition.
    pub struct SolarPanelGroup {
        ///The name of this solar panel group.  The group name must be unique within this model, and may not contain spaces.
        pub name: String,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The percentage, from 0.0 to 100.0, of how efficiently the solar cells convert solar to electrical energy.
        pub efficiency: f64,
    }
    impl crate::GltfObject for SolarPanelGroup {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use solar_panel_group::SolarPanelGroup;
