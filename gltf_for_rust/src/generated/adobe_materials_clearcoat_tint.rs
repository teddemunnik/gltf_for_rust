#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the colour tint of the clearcoat.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "clearcoatTintFactor")]
            #[serde(default = "get_default_clearcoat_tint_factor")]
            ///The colour of light allowed to be transmitted through the clearcoat layer of the material. A value of black means no light passes through. A value of white means all light passes through. These values are linear.
            pub clearcoat_tint_factor: [f64; 3usize],
            #[serde(rename = "clearcoatTintTexture")]
            #[serde(default)]
            ///The clearcoat layer tint texture.  The values are stored in sRGB.  Assume white colour if no texture is supplied.
            pub clearcoat_tint_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
        fn get_default_clearcoat_tint_factor() -> [f64; 3usize] {
            [1f64, 1f64, 1f64]
        }
    }
    pub use extension::Extension;
    impl crate::GltfExtension for Extension {
        fn extension_name() -> &'static str {
            "ADOBE_materials_clearcoat_tint"
        }
    }
}
