#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables the use of IES light profiles.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            pub lights: Vec<Map<String, Value>>,
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
            "EXT_lights_ies"
        }
    }
}
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default = "get_default_color")]
            ///RGB value for the light's color in linear space.
            pub color: [f64; 3usize],
            ///The id of the light profile referenced by this node.
            pub light: i64,
            #[serde(default = "get_default_multiplier")]
            ///Non-negative factor to scale the light's intensity.
            pub multiplier: f64,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_color() -> [f64; 3usize] {
            [1f64, 1f64, 1f64]
        }
        fn get_default_multiplier() -> f64 {
            1f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "EXT_lights_ies"
        }
    }
}
