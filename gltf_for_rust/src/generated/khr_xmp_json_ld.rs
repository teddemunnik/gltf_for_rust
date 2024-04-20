#![allow(clippy::all, unused_imports)]
mod gltf_khr_xmp_json_ld {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
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
    #[derive(Serialize, Deserialize, Debug)]
    ///Metadata about the glTF asset.
    pub struct GltfKhrXmpJsonLd {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        pub packets: Vec<Packet>,
    }
    impl crate::GltfExtension for GltfKhrXmpJsonLd {
        fn extension_name() -> &'static str {
            "KHR_xmp_json_ld"
        }
    }
    impl crate::GltfObject for GltfKhrXmpJsonLd {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_khr_xmp_json_ld::GltfKhrXmpJsonLd;
