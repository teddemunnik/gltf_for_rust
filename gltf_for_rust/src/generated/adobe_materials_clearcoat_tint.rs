#![allow(clippy::all, unused_imports)]
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that defines the colour tint of the clearcoat.
        pub struct Extension {
            #[serde(rename = "clearcoatTintFactor")]
            #[serde(default = "get_default_clearcoat_tint_factor")]
            ///The transmittance of the clearcoat layer.
            pub clearcoat_tint_factor: [f64; 3usize],
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(rename = "clearcoatTintTexture")]
            #[serde(default)]
            ///The clearcoat layer tint texture.
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
