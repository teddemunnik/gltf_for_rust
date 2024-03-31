#![allow(clippy::all, unused_imports)]
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            ///The id of the light referenced by this node.
            pub light: i64,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
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
            "KHR_lights_punctual"
        }
    }
}
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            pub lights: Vec<crate::generated::khr_lights_punctual::Light>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
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
            "KHR_lights_punctual"
        }
    }
}
mod light {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A directional, point, or spot light.
    pub struct Light {
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default = "get_default_color")]
        ///Color of the light source.
        pub color: [f64; 3usize],
        #[serde(default)]
        ///A distance cutoff at which the light's intensity may be considered to have reached zero.
        pub range: Option<f64>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        pub spot: Option<crate::generated::khr_lights_punctual::LightSpot>,
        #[serde(rename = "type")]
        ///Specifies the light type.
        pub ty: String,
        #[serde(default = "get_default_intensity")]
        ///Intensity of the light source. `point` and `spot` lights use luminous intensity in candela (lm/sr) while `directional` lights use illuminance in lux (lm/m^2)
        pub intensity: f64,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
    }
    impl crate::GltfObject for Light {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_color() -> [f64; 3usize] {
        [1f64, 1f64, 1f64]
    }
    fn get_default_intensity() -> f64 {
        1f64
    }
}
pub use light::Light;
mod light_spot {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct LightSpot {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "outerConeAngle")]
        #[serde(default = "get_default_outer_cone_angle")]
        ///Angle in radians from centre of spotlight where falloff ends.
        pub outer_cone_angle: f64,
        #[serde(rename = "innerConeAngle")]
        #[serde(default = "get_default_inner_cone_angle")]
        ///Angle in radians from centre of spotlight where falloff begins.
        pub inner_cone_angle: f64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for LightSpot {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_outer_cone_angle() -> f64 {
        0.7853981633974483f64
    }
    fn get_default_inner_cone_angle() -> f64 {
        0f64
    }
}
pub use light_spot::LightSpot;
