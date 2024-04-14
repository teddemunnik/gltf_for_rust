#![allow(clippy::all, unused_imports)]
pub mod texture_info {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables shifting and scaling UV coordinates on a per-texture basis
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///The offset of the UV coordinate origin as a factor of the texture dimensions.
            pub offset: Option<[f64; 2usize]>,
            #[serde(default)]
            ///Rotate the UVs by this many radians counter-clockwise around the origin.
            pub rotation: Option<f64>,
            #[serde(default)]
            ///The scale factor applied to the components of the UV coordinates.
            pub scale: Option<[f64; 2usize]>,
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
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "KHR_texture_transform"
        }
    }
}
