#![allow(clippy::all, unused_imports)]
mod gltf_ext_texture_webp {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension to specify textures using the WebP image format.
    pub struct GltfExtTextureWebp {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the images node which points to a WebP image.
        pub source: Option<i64>,
    }
    impl crate::GltfExtension for GltfExtTextureWebp {
        fn extension_name() -> &'static str {
            "EXT_texture_webp"
        }
    }
    impl crate::GltfObject for GltfExtTextureWebp {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf_ext_texture_webp::GltfExtTextureWebp;
