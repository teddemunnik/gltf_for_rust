#![allow(clippy::all, unused_imports)]
pub mod scene {
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
            #[serde(default = "get_default_primitive_count")]
            ///The number of distinct primitives recursively contained in this scene.
            pub primitive_count: f64,
            #[serde(rename = "sceneBounds")]
            #[serde(default)]
            ///The bounding box of this scene, in static geometry scene-space coordinates.
            pub scene_bounds: Option<
                crate::generated::fb_geometry_metadata::SceneBounds,
            >,
            #[serde(rename = "vertexCount")]
            #[serde(default = "get_default_vertex_count")]
            ///The number of distinct vertices recursively contained in this scene.
            pub vertex_count: f64,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_primitive_count() -> f64 {
            0f64
        }
        fn get_default_vertex_count() -> f64 {
            0f64
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "FB_geometry_metadata"
        }
    }
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
