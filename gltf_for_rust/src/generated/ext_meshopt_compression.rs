#![allow(clippy::all, unused_imports)]
pub mod buffer {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///Compressed data for bufferView.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default = "get_default_fallback")]
            ///Set to true to indicate that the buffer is only referenced by bufferViews that have EXT_meshopt_compression extension and as such doesn't need to be loaded.
            pub fallback: bool,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_fallback() -> bool {
            false
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "EXT_meshopt_compression"
        }
    }
}
pub mod buffer_view {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        #[derive(Default)]
        pub enum Filter {
            #[serde(rename = "NONE")]
            #[default]
            None,
            #[serde(rename = "OCTAHEDRAL")]
            Octahedral,
            #[serde(rename = "QUATERNION")]
            Quaternion,
            #[serde(rename = "EXPONENTIAL")]
            Exponential,
        }
        #[derive(Serialize, Deserialize, Debug)]
        pub enum Mode {
            #[serde(rename = "ATTRIBUTES")]
            Attributes,
            #[serde(rename = "TRIANGLES")]
            Triangles,
            #[serde(rename = "INDICES")]
            Indices,
        }
        #[derive(Serialize, Deserialize, Debug)]
        ///Compressed data for bufferView.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            ///The index of the buffer with compressed data.
            pub buffer: i64,
            #[serde(rename = "byteLength")]
            ///The length of the compressed data in bytes.
            pub byte_length: i64,
            #[serde(rename = "byteOffset")]
            #[serde(default = "get_default_byte_offset")]
            ///The offset into the buffer in bytes.
            pub byte_offset: i64,
            #[serde(rename = "byteStride")]
            ///The stride, in bytes.
            pub byte_stride: i64,
            ///The number of elements.
            pub count: i64,
            #[serde(default = "get_default_filter")]
            ///The compression filter.
            pub filter: Filter,
            ///The compression mode.
            pub mode: Mode,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_byte_offset() -> i64 {
            0i64
        }
        fn get_default_filter() -> Filter {
            Filter::default()
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "EXT_meshopt_compression"
        }
    }
}
