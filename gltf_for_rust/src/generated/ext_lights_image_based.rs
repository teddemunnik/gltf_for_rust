#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            pub lights: Vec<crate::generated::ext_lights_image_based::Light>,
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
            "EXT_lights_image_based"
        }
    }
}
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
            ///The id of the light referenced by this scene.
            pub light: i64,
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
            "EXT_lights_image_based"
        }
    }
}
mod light {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An image-based lighting environment.
    pub struct Light {
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "irradianceCoefficients")]
        ///Declares spherical harmonic coefficients for irradiance up to l=2. This is a 9x3 array.
        pub irradiance_coefficients: [[f64; 3usize]; 9usize],
        #[serde(default = "get_default_rotation")]
        ///Quaternion that represents the rotation of the IBL environment.
        pub rotation: [f64; 4usize],
        #[serde(rename = "specularImageSize")]
        ///The dimension (in pixels) of the first specular mip. This is needed to determine, pre-load, the total number of mips needed.
        pub specular_image_size: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default = "get_default_intensity")]
        ///Brightness multiplier for environment.
        pub intensity: f64,
        #[serde(rename = "specularImages")]
        ///Declares an array of the first N mips of the prefiltered cubemap. Each mip is, in turn, defined with an array of 6 images, one for each cube face. i.e. this is an Nx6 array.
        pub specular_images: Vec<[i64; 6usize]>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
    }
    impl crate::GltfObject for Light {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_rotation() -> [f64; 4usize] {
        [0f64, 0f64, 0f64, 1f64]
    }
    fn get_default_intensity() -> f64 {
        1f64
    }
}
pub use light::Light;
