#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///Metadata about the glTF asset.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            pub packets: Vec<crate::generated::khr_xmp_json_ld::Packet>,
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
            "KHR_xmp_json_ld"
        }
    }
}
mod packet {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Dictionary of XMP metadata properties. Property names take the form `xmp_namespace_name:property_name`
    pub struct Packet {
        #[serde(rename = "@context")]
        ///Dictionary mapping XMP namespace names to the URI where they are defined
        pub context: Map<String, Value>,
    }
}
pub use packet::Packet;
