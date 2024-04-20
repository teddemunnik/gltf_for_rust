#![allow(clippy::all, unused_imports)]
mod texture_khr_texture_basisu {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///glTF extension to specify textures using the KTX v2 images with Basis Universal supercompression.
    pub struct TextureKhrTextureBasisu {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the image which points to a KTX v2 resource with Basis Universal supercompression.
        pub source: Option<i64>,
    }
    impl crate::GltfExtension for TextureKhrTextureBasisu {
        fn extension_name() -> &'static str {
            "KHR_texture_basisu"
        }
    }
    impl crate::GltfObject for TextureKhrTextureBasisu {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use texture_khr_texture_basisu::TextureKhrTextureBasisu;
