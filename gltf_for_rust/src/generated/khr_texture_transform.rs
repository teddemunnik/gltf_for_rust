#![allow(clippy::all, unused_imports)]
pub mod texture_info {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables shifting and scaling UV coordinates on a per-texture basis
        pub struct Extension {
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default = "get_default_offset")]
            ///The offset of the UV coordinate origin as a factor of the texture dimensions.
            pub offset: [f64; 2usize],
            #[serde(default = "get_default_rotation")]
            ///Rotate the UVs by this many radians counter-clockwise around the origin.
            pub rotation: f64,
            #[serde(default = "get_default_scale")]
            ///The scale factor applied to the components of the UV coordinates.
            pub scale: [f64; 2usize],
            #[serde(rename = "texCoord")]
            #[serde(default)]
            ///Overrides the textureInfo texCoord value if supplied, and if this extension is supported.
            pub tex_coord: Option<i64>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_offset() -> [f64; 2usize] {
            [0f64, 0f64]
        }
        fn get_default_rotation() -> f64 {
            0f64
        }
        fn get_default_scale() -> [f64; 2usize] {
            [1f64, 1f64]
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_texture_transform"
        }
    }
}
