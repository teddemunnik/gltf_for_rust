#![allow(clippy::all, unused_imports)]
pub mod node {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Extension {
            #[serde(default = "get_default_color")]
            ///RGB value for the light's color in linear space.
            pub color: [f64; 3usize],
            ///The id of the light profile referenced by this node.
            pub light: i64,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default = "get_default_multiplier")]
            ///Non-negative factor to scale the light's intensity.
            pub multiplier: f64,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
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
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables the use of IES light profiles.
        pub struct Extension {
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            pub lights: Vec<crate::generated::ext_lights_ies::LightProfile>,
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
            "EXT_lights_ies"
        }
    }
}
mod light_profile {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum MimeType {
        #[serde(rename = "application/x-ies-lm-63")]
        ApplicationXIesLm63,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///An IES light profile.
    pub struct LightProfile {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The index of the bufferView that contains the IES light profile. This field **MUST NOT** be defined when `uri` is defined.
        pub buffer_view: Option<i64>,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        ///The light profile's media type. This field **MUST** be defined when `bufferView` is defined.
        pub mime_ty: Option<MimeType>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The URI (or IRI) of the light profile.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for LightProfile {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use light_profile::LightProfile;
