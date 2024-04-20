#![allow(clippy::all, unused_imports)]
mod scene {
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
            #[serde(rename = "primitiveCount")]
            #[serde(default)]
            ///The number of distinct primitives recursively contained in this scene.
            pub primitive_count: Option<f64>,
            #[serde(rename = "sceneBounds")]
            #[serde(default)]
            ///The bounding box of this scene, in static geometry scene-space coordinates.
            pub scene_bounds: Option<
                crate::generated::fb_geometry_metadata::SceneBounds,
            >,
            #[serde(rename = "vertexCount")]
            #[serde(default)]
            ///The number of distinct vertices recursively contained in this scene.
            pub vertex_count: Option<f64>,
        }
        impl crate::GltfExtension for Extension {
            fn extension_name() -> &'static str {
                "FB_geometry_metadata"
            }
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
}
mod scene_bounds {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SceneBounds {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The bounding box corner with the numerically highest scene-space coordinates
        pub max: [f64; 3usize],
        ///The bounding box corner with the numerically lowest scene-space coordinates
        pub min: [f64; 3usize],
    }
    impl crate::GltfObject for SceneBounds {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use scene_bounds::SceneBounds;
