#![allow(clippy::all, unused_imports)]
mod gltf {
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
            #[serde(rename = "clearcoatIor")]
            #[serde(default)]
            ///The clearcoat layer's index of refraction.
            pub clearcoat_ior: Option<f64>,
            #[serde(rename = "clearcoatSpecularFactor")]
            #[serde(default)]
            ///The clearcoat layer's specular intensity.
            pub clearcoat_specular_factor: Option<f64>,
            #[serde(rename = "clearcoatSpecularTexture")]
            #[serde(default)]
            ///The clearcoat layer's specular intensity texture. These values are sampled from the B channel.
            pub clearcoat_specular_texture: Option<crate::generated::gltf::TextureInfo>,
        }
        impl crate::GltfExtension for Extension {
            fn extension_name() -> &'static str {
                "ADOBE_materials_clearcoat_specular"
            }
        }
        impl crate::GltfObject for Extension {
            fn extensions(&self) -> &Option<Map<String, Value>> {
                &self.extensions
            }
        }
    }
    pub use extension::Extension;
}
