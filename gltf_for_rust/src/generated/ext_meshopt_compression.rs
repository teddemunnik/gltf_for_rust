#![allow(clippy::all, unused_imports)]
mod buffer_ext_meshopt_compression {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Compressed data for bufferView.
    pub struct BufferExtMeshoptCompression {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///Set to true to indicate that the buffer is only referenced by bufferViews that have EXT_meshopt_compression extension and as such doesn't need to be loaded.
        pub fallback: Option<bool>,
    }
    impl crate::GltfExtension for BufferExtMeshoptCompression {
        fn extension_name() -> &'static str {
            "EXT_meshopt_compression"
        }
    }
    impl crate::GltfObject for BufferExtMeshoptCompression {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use buffer_ext_meshopt_compression::BufferExtMeshoptCompression;
mod buffer_view_ext_meshopt_compression {
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
    pub struct BufferViewExtMeshoptCompression {
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
        #[serde(default)]
        ///The offset into the buffer in bytes.
        pub byte_offset: Option<i64>,
        #[serde(rename = "byteStride")]
        ///The stride, in bytes.
        pub byte_stride: i64,
        ///The number of elements.
        pub count: i64,
        #[serde(default)]
        ///The compression filter.
        pub filter: Option<Filter>,
        ///The compression mode.
        pub mode: Mode,
    }
    impl crate::GltfExtension for BufferViewExtMeshoptCompression {
        fn extension_name() -> &'static str {
            "EXT_meshopt_compression"
        }
    }
    impl crate::GltfObject for BufferViewExtMeshoptCompression {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use buffer_view_ext_meshopt_compression::BufferViewExtMeshoptCompression;
