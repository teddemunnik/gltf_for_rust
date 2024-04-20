#![allow(clippy::all, unused_imports)]
mod skin {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Joints and matrices defining a skin.
    pub struct Skin {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "inverseBindMatrices")]
        #[serde(default)]
        ///The index of the accessor containing the floating-point 4x4 inverse-bind matrices. Its `accessor.count` property **MUST** be greater than or equal to the number of elements of the `joints` array. When undefined, each matrix is a 4x4 identity matrix.
        pub inverse_bind_matrices: Option<i64>,
        ///Indices of skeleton nodes, used as joints in this skin.
        pub joints: Vec<i64>,
        #[serde(default)]
        ///The index of the node used as a skeleton root. The node **MUST** be the closest common root of the joints hierarchy or a direct or indirect parent node of the closest common root.
        pub skeleton: Option<i64>,
    }
    impl crate::GltfObject for Skin {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use skin::Skin;
mod material {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Default)]
    pub enum AlphaMode {
        #[serde(rename = "OPAQUE")]
        #[default]
        Opaque,
        #[serde(rename = "MASK")]
        Mask,
        #[serde(rename = "BLEND")]
        Blend,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///The material appearance of a primitive.
    pub struct Material {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "alphaCutoff")]
        #[serde(default)]
        ///Specifies the cutoff threshold when in `MASK` alpha mode. If the alpha value is greater than or equal to this value then it is rendered as fully opaque, otherwise, it is rendered as fully transparent. A value greater than `1.0` will render the entire material as fully transparent. This value **MUST** be ignored for other alpha modes. When `alphaMode` is not defined, this value **MUST NOT** be defined.
        pub alpha_cutoff: Option<f64>,
        #[serde(rename = "alphaMode")]
        #[serde(default)]
        ///The material's alpha rendering mode enumeration specifying the interpretation of the alpha value of the base color.
        pub alpha_mode: Option<AlphaMode>,
        #[serde(rename = "doubleSided")]
        #[serde(default)]
        ///Specifies whether the material is double sided. When this value is false, back-face culling is enabled. When this value is true, back-face culling is disabled and double-sided lighting is enabled. The back-face **MUST** have its normals reversed before the lighting equation is evaluated.
        pub double_sided: Option<bool>,
        #[serde(rename = "emissiveFactor")]
        #[serde(default)]
        ///The factors for the emissive color of the material. This value defines linear multipliers for the sampled texels of the emissive texture.
        pub emissive_factor: Option<[f64; 3usize]>,
        #[serde(rename = "emissiveTexture")]
        #[serde(default)]
        ///The emissive texture. It controls the color and intensity of the light being emitted by the material. This texture contains RGB components encoded with the sRGB transfer function. If a fourth component (A) is present, it **MUST** be ignored. When undefined, the texture **MUST** be sampled as having `1.0` in RGB components.
        pub emissive_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "normalTexture")]
        #[serde(default)]
        ///The tangent space normal texture. The texture encodes RGB components with linear transfer function. Each texel represents the XYZ components of a normal vector in tangent space. The normal vectors use the convention +X is right and +Y is up. +Z points toward the viewer. If a fourth component (A) is present, it **MUST** be ignored. When undefined, the material does not have a tangent space normal texture.
        pub normal_texture: Option<crate::generated::gltf::MaterialNormalTextureInfo>,
        #[serde(rename = "occlusionTexture")]
        #[serde(default)]
        ///The occlusion texture. The occlusion values are linearly sampled from the R channel. Higher values indicate areas that receive full indirect lighting and lower values indicate no indirect lighting. If other channels are present (GBA), they **MUST** be ignored for occlusion calculations. When undefined, the material does not have an occlusion texture.
        pub occlusion_texture: Option<
            crate::generated::gltf::MaterialOcclusionTextureInfo,
        >,
        #[serde(rename = "pbrMetallicRoughness")]
        #[serde(default)]
        ///A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply.
        pub pbr_metallic_roughness: Option<
            crate::generated::gltf::MaterialPbrMetallicRoughness,
        >,
    }
    impl crate::GltfObject for Material {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material::Material;
mod camera_perspective {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A perspective camera containing properties to create a perspective projection matrix.
    pub struct CameraPerspective {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "aspectRatio")]
        #[serde(default)]
        ///The floating-point aspect ratio of the field of view. When undefined, the aspect ratio of the rendering viewport **MUST** be used.
        pub aspect_ratio: Option<f64>,
        ///The floating-point vertical field of view in radians. This value **SHOULD** be less than π.
        pub yfov: f64,
        #[serde(default)]
        ///The floating-point distance to the far clipping plane. When defined, `zfar` **MUST** be greater than `znear`. If `zfar` is undefined, client implementations **SHOULD** use infinite projection matrix.
        pub zfar: Option<f64>,
        ///The floating-point distance to the near clipping plane.
        pub znear: f64,
    }
    impl crate::GltfObject for CameraPerspective {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera_perspective::CameraPerspective;
mod material_normal_texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MaterialNormalTextureInfo {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of the texture.
        pub index: i64,
        #[serde(rename = "texCoord")]
        #[serde(default)]
        ///This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
        pub tex_coord: Option<i64>,
        #[serde(default)]
        ///The scalar parameter applied to each normal vector of the texture. This value scales the normal vector in X and Y directions using the formula: `scaledNormal =  normalize((<sampled normal texture value> * 2.0 - 1.0) * vec3(<normal scale>, <normal scale>, 1.0))`.
        pub scale: Option<f64>,
    }
    impl crate::GltfObject for MaterialNormalTextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_normal_texture_info::MaterialNormalTextureInfo;
mod animation {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A keyframe animation.
    pub struct Animation {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        ///An array of animation channels. An animation channel combines an animation sampler with a target property being animated. Different channels of the same animation **MUST NOT** have the same targets.
        pub channels: Vec<crate::generated::gltf::AnimationChannel>,
        ///An array of animation samplers. An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
        pub samplers: Vec<crate::generated::gltf::AnimationSampler>,
    }
    impl crate::GltfObject for Animation {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation::Animation;
mod camera {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Type {
        #[serde(rename = "perspective")]
        Perspective,
        #[serde(rename = "orthographic")]
        Orthographic,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///A camera's projection.  A node **MAY** reference a camera to apply a transform to place the camera in the scene.
    pub struct Camera {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(default)]
        ///An orthographic camera containing properties to create an orthographic projection matrix. This property **MUST NOT** be defined when `perspective` is defined.
        pub orthographic: Option<crate::generated::gltf::CameraOrthographic>,
        #[serde(default)]
        ///A perspective camera containing properties to create a perspective projection matrix. This property **MUST NOT** be defined when `orthographic` is defined.
        pub perspective: Option<crate::generated::gltf::CameraPerspective>,
        #[serde(rename = "type")]
        ///Specifies if the camera uses a perspective or orthographic projection.  Based on this, either the camera's `perspective` or `orthographic` property **MUST** be defined.
        pub ty: Type,
    }
    impl crate::GltfObject for Camera {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera::Camera;
mod material_occlusion_texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MaterialOcclusionTextureInfo {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of the texture.
        pub index: i64,
        #[serde(rename = "texCoord")]
        #[serde(default)]
        ///This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
        pub tex_coord: Option<i64>,
        #[serde(default)]
        ///A scalar parameter controlling the amount of occlusion applied. A value of `0.0` means no occlusion. A value of `1.0` means full occlusion. This value affects the final occlusion value as: `1.0 + strength * (<sampled occlusion texture value> - 1.0)`.
        pub strength: Option<f64>,
    }
    impl crate::GltfObject for MaterialOcclusionTextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_occlusion_texture_info::MaterialOcclusionTextureInfo;
mod image {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum MimeType {
        #[serde(rename = "image/jpeg")]
        ImageJpeg,
        #[serde(rename = "image/png")]
        ImagePng,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///Image data used to create a texture. Image **MAY** be referenced by an URI (or IRI) or a buffer view index.
    pub struct Image {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The index of the bufferView that contains the image. This field **MUST NOT** be defined when `uri` is defined.
        pub buffer_view: Option<i64>,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        ///The image's media type. This field **MUST** be defined when `bufferView` is defined.
        pub mime_type: Option<MimeType>,
        #[serde(default)]
        ///The URI (or IRI) of the image.  Relative paths are relative to the current glTF asset.  Instead of referencing an external file, this field **MAY** contain a `data:`-URI. This field **MUST NOT** be defined when `bufferView` is defined.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for Image {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use image::Image;
mod texture {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A texture and its sampler.
    pub struct Texture {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(default)]
        ///The index of the sampler used by this texture. When undefined, a sampler with repeat wrapping and auto filtering **SHOULD** be used.
        pub sampler: Option<i64>,
        #[serde(default)]
        ///The index of the image used by this texture. When undefined, an extension or other mechanism **SHOULD** supply an alternate texture source, otherwise behavior is undefined.
        pub source: Option<i64>,
    }
    impl crate::GltfObject for Texture {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use texture::Texture;
mod node {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present.
    pub struct Node {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(default)]
        ///The index of the camera referenced by this node.
        pub camera: Option<i64>,
        #[serde(default)]
        ///The indices of this node's children.
        pub children: Vec<i64>,
        #[serde(default)]
        ///A floating-point 4x4 transformation matrix stored in column-major order.
        pub matrix: Option<[f64; 16usize]>,
        #[serde(default)]
        ///The index of the mesh in this node.
        pub mesh: Option<i64>,
        #[serde(default)]
        ///The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
        pub rotation: Option<[f64; 4usize]>,
        #[serde(default)]
        ///The node's non-uniform scale, given as the scaling factors along the x, y, and z axes.
        pub scale: Option<[f64; 3usize]>,
        #[serde(default)]
        ///The index of the skin referenced by this node. When a skin is referenced by a node within a scene, all joints used by the skin **MUST** belong to the same scene. When defined, `mesh` **MUST** also be defined.
        pub skin: Option<i64>,
        #[serde(default)]
        ///The node's translation along the x, y, and z axes.
        pub translation: Option<[f64; 3usize]>,
        #[serde(default)]
        ///The weights of the instantiated morph target. The number of array elements **MUST** match the number of morph targets of the referenced mesh. When defined, `mesh` **MUST** also be defined.
        pub weights: Vec<f64>,
    }
    impl crate::GltfObject for Node {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use node::Node;
mod camera_orthographic {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An orthographic camera containing properties to create an orthographic projection matrix.
    pub struct CameraOrthographic {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The floating-point horizontal magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
        pub xmag: f64,
        ///The floating-point vertical magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
        pub ymag: f64,
        ///The floating-point distance to the far clipping plane. This value **MUST NOT** be equal to zero. `zfar` **MUST** be greater than `znear`.
        pub zfar: f64,
        ///The floating-point distance to the near clipping plane.
        pub znear: f64,
    }
    impl crate::GltfObject for CameraOrthographic {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera_orthographic::CameraOrthographic;
mod material_pbr_metallic_roughness {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology.
    pub struct MaterialPbrMetallicRoughness {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "baseColorFactor")]
        #[serde(default)]
        ///The factors for the base color of the material. This value defines linear multipliers for the sampled texels of the base color texture.
        pub base_color_factor: Option<[f64; 4usize]>,
        #[serde(rename = "baseColorTexture")]
        #[serde(default)]
        ///The base color texture. The first three components (RGB) **MUST** be encoded with the sRGB transfer function. They specify the base color of the material. If the fourth component (A) is present, it represents the linear alpha coverage of the material. Otherwise, the alpha coverage is equal to `1.0`. The `material.alphaMode` property specifies how alpha is interpreted. The stored texels **MUST NOT** be premultiplied. When undefined, the texture **MUST** be sampled as having `1.0` in all components.
        pub base_color_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "metallicFactor")]
        #[serde(default)]
        ///The factor for the metalness of the material. This value defines a linear multiplier for the sampled metalness values of the metallic-roughness texture.
        pub metallic_factor: Option<f64>,
        #[serde(rename = "metallicRoughnessTexture")]
        #[serde(default)]
        ///The metallic-roughness texture. The metalness values are sampled from the B channel. The roughness values are sampled from the G channel. These values **MUST** be encoded with a linear transfer function. If other channels are present (R or A), they **MUST** be ignored for metallic-roughness calculations. When undefined, the texture **MUST** be sampled as having `1.0` in G and B components.
        pub metallic_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "roughnessFactor")]
        #[serde(default)]
        ///The factor for the roughness of the material. This value defines a linear multiplier for the sampled roughness values of the metallic-roughness texture.
        pub roughness_factor: Option<f64>,
    }
    impl crate::GltfObject for MaterialPbrMetallicRoughness {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use material_pbr_metallic_roughness::MaterialPbrMetallicRoughness;
mod buffer {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A buffer points to binary geometry, animation, or skins.
    pub struct Buffer {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "byteLength")]
        ///The length of the buffer in bytes.
        pub byte_length: i64,
        #[serde(default)]
        ///The URI (or IRI) of the buffer.  Relative paths are relative to the current glTF asset.  Instead of referencing an external file, this field **MAY** contain a `data:`-URI.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for Buffer {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use buffer::Buffer;
mod animation_channel {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An animation channel combines an animation sampler with a target property being animated.
    pub struct AnimationChannel {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of a sampler in this animation used to compute the value for the target, e.g., a node's translation, rotation, or scale (TRS).
        pub sampler: i64,
        ///The descriptor of the animated property.
        pub target: crate::generated::gltf::AnimationChannelTarget,
    }
    impl crate::GltfObject for AnimationChannel {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation_channel::AnimationChannel;
mod accessor_sparse {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Sparse storage of accessor values that deviate from their initialization value.
    pub struct AccessorSparse {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///Number of deviating accessor values stored in the sparse array.
        pub count: i64,
        ///An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase.
        pub indices: crate::generated::gltf::AccessorSparseIndices,
        ///An object pointing to a buffer view containing the deviating accessor values.
        pub values: crate::generated::gltf::AccessorSparseValues,
    }
    impl crate::GltfObject for AccessorSparse {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use accessor_sparse::AccessorSparse;
mod accessor_sparse_indices {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `accessor.sparse.count`. Indices **MUST** strictly increase.
    pub struct AccessorSparseIndices {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "bufferView")]
        ///The index of the buffer view with sparse indices. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined. The buffer view and the optional `byteOffset` **MUST** be aligned to the `componentType` byte length.
        pub buffer_view: i64,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        ///The offset relative to the start of the buffer view in bytes.
        pub byte_offset: Option<i64>,
        #[serde(rename = "componentType")]
        ///The indices data type.
        pub component_type: i64,
    }
    impl crate::GltfObject for AccessorSparseIndices {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use accessor_sparse_indices::AccessorSparseIndices;
mod mesh {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A set of primitives to be rendered.  Its global transform is defined by a node that references it.
    pub struct Mesh {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        ///An array of primitives, each defining geometry to be rendered.
        pub primitives: Vec<crate::generated::gltf::MeshPrimitive>,
        #[serde(default)]
        ///Array of weights to be applied to the morph targets. The number of array elements **MUST** match the number of morph targets.
        pub weights: Vec<f64>,
    }
    impl crate::GltfObject for Mesh {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use mesh::Mesh;
mod mesh_primitive {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Geometry to be rendered with the given material.
    pub struct MeshPrimitive {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///A plain JSON object, where each key corresponds to a mesh attribute semantic and each value is the index of the accessor containing attribute's data.
        pub attributes: Map<String, Value>,
        #[serde(default)]
        ///The index of the accessor that contains the vertex indices.  When this is undefined, the primitive defines non-indexed geometry.  When defined, the accessor **MUST** have `SCALAR` type and an unsigned integer component type.
        pub indices: Option<i64>,
        #[serde(default)]
        ///The index of the material to apply to this primitive when rendering.
        pub material: Option<i64>,
        #[serde(default)]
        ///The topology type of primitives to render.
        pub mode: Option<i64>,
        #[serde(default)]
        ///An array of morph targets.
        pub targets: Vec<Map<String, Value>>,
    }
    impl crate::GltfObject for MeshPrimitive {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use mesh_primitive::MeshPrimitive;
mod animation_channel_target {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Path {
        #[serde(rename = "translation")]
        Translation,
        #[serde(rename = "rotation")]
        Rotation,
        #[serde(rename = "scale")]
        Scale,
        #[serde(rename = "weights")]
        Weights,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///The descriptor of the animated property.
    pub struct AnimationChannelTarget {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the node to animate. When undefined, the animated object **MAY** be defined by an extension.
        pub node: Option<i64>,
        ///The name of the node's TRS property to animate, or the `"weights"` of the Morph Targets it instantiates. For the `"translation"` property, the values that are provided by the sampler are the translation along the X, Y, and Z axes. For the `"rotation"` property, the values are a quaternion in the order (x, y, z, w), where w is the scalar. For the `"scale"` property, the values are the scaling factors along the X, Y, and Z axes.
        pub path: Path,
    }
    impl crate::GltfObject for AnimationChannelTarget {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation_channel_target::AnimationChannelTarget;
mod buffer_view {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A view into a buffer generally representing a subset of the buffer.
    pub struct BufferView {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        ///The index of the buffer.
        pub buffer: i64,
        #[serde(rename = "byteLength")]
        ///The length of the bufferView in bytes.
        pub byte_length: i64,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        ///The offset into the buffer in bytes.
        pub byte_offset: Option<i64>,
        #[serde(rename = "byteStride")]
        #[serde(default)]
        ///The stride, in bytes, between vertex attributes.  When this is not defined, data is tightly packed. When two or more accessors use the same buffer view, this field **MUST** be defined.
        pub byte_stride: Option<i64>,
        #[serde(default)]
        ///The hint representing the intended GPU buffer type to use with this buffer view.
        pub target: Option<i64>,
    }
    impl crate::GltfObject for BufferView {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use buffer_view::BufferView;
mod accessor_sparse_values {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An object pointing to a buffer view containing the deviating accessor values. The number of elements is equal to `accessor.sparse.count` times number of components. The elements have the same component type as the base accessor. The elements are tightly packed. Data **MUST** be aligned following the same rules as the base accessor.
    pub struct AccessorSparseValues {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "bufferView")]
        ///The index of the bufferView with sparse values. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined.
        pub buffer_view: i64,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        ///The offset relative to the start of the bufferView in bytes.
        pub byte_offset: Option<i64>,
    }
    impl crate::GltfObject for AccessorSparseValues {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use accessor_sparse_values::AccessorSparseValues;
mod animation_sampler {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Default)]
    pub enum Interpolation {
        #[serde(rename = "LINEAR")]
        #[default]
        Linear,
        #[serde(rename = "STEP")]
        Step,
        #[serde(rename = "CUBICSPLINE")]
        Cubicspline,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
    pub struct AnimationSampler {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of an accessor containing keyframe timestamps. The accessor **MUST** be of scalar type with floating-point components. The values represent time in seconds with `time[0] >= 0.0`, and strictly increasing values, i.e., `time[n + 1] > time[n]`.
        pub input: i64,
        #[serde(default)]
        ///Interpolation algorithm.
        pub interpolation: Option<Interpolation>,
        ///The index of an accessor, containing keyframe output values.
        pub output: i64,
    }
    impl crate::GltfObject for AnimationSampler {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation_sampler::AnimationSampler;
mod texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Reference to a texture.
    pub struct TextureInfo {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of the texture.
        pub index: i64,
        #[serde(rename = "texCoord")]
        #[serde(default)]
        ///This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
        pub tex_coord: Option<i64>,
    }
    impl crate::GltfObject for TextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use texture_info::TextureInfo;
mod sampler {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Texture sampler properties for filtering and wrapping modes.
    pub struct Sampler {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "magFilter")]
        #[serde(default)]
        ///Magnification filter.
        pub mag_filter: Option<i64>,
        #[serde(rename = "minFilter")]
        #[serde(default)]
        ///Minification filter.
        pub min_filter: Option<i64>,
        #[serde(rename = "wrapS")]
        #[serde(default)]
        ///S (U) wrapping mode.  All valid values correspond to WebGL enums.
        pub wrap_s: Option<i64>,
        #[serde(rename = "wrapT")]
        #[serde(default)]
        ///T (V) wrapping mode.
        pub wrap_t: Option<i64>,
    }
    impl crate::GltfObject for Sampler {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use sampler::Sampler;
mod asset {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Metadata about the glTF asset.
    pub struct Asset {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///A copyright message suitable for display to credit the content creator.
        pub copyright: Option<String>,
        #[serde(default)]
        ///Tool that generated this glTF model.  Useful for debugging.
        pub generator: Option<String>,
        #[serde(rename = "minVersion")]
        #[serde(default)]
        ///The minimum glTF version in the form of `<major>.<minor>` that this asset targets. This property **MUST NOT** be greater than the asset version.
        pub min_version: Option<String>,
        ///The glTF version in the form of `<major>.<minor>` that this asset targets.
        pub version: String,
    }
    impl crate::GltfObject for Asset {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use asset::Asset;
mod gltf {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///The root object for a glTF asset.
    pub struct Gltf {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///An array of accessors.  An accessor is a typed view into a bufferView.
        pub accessors: Vec<crate::generated::gltf::Accessor>,
        #[serde(default)]
        ///An array of keyframe animations.
        pub animations: Vec<crate::generated::gltf::Animation>,
        ///Metadata about the glTF asset.
        pub asset: crate::generated::gltf::Asset,
        #[serde(rename = "bufferViews")]
        #[serde(default)]
        ///An array of bufferViews.  A bufferView is a view into a buffer generally representing a subset of the buffer.
        pub buffer_views: Vec<crate::generated::gltf::BufferView>,
        #[serde(default)]
        ///An array of buffers.  A buffer points to binary geometry, animation, or skins.
        pub buffers: Vec<crate::generated::gltf::Buffer>,
        #[serde(default)]
        ///An array of cameras.  A camera defines a projection matrix.
        pub cameras: Vec<crate::generated::gltf::Camera>,
        #[serde(rename = "extensionsRequired")]
        #[serde(default)]
        ///Names of glTF extensions required to properly load this asset.
        pub extensions_required: Vec<String>,
        #[serde(rename = "extensionsUsed")]
        #[serde(default)]
        ///Names of glTF extensions used in this asset.
        pub extensions_used: Vec<String>,
        #[serde(default)]
        ///An array of images.  An image defines data used to create a texture.
        pub images: Vec<crate::generated::gltf::Image>,
        #[serde(default)]
        ///An array of materials.  A material defines the appearance of a primitive.
        pub materials: Vec<crate::generated::gltf::Material>,
        #[serde(default)]
        ///An array of meshes.  A mesh is a set of primitives to be rendered.
        pub meshes: Vec<crate::generated::gltf::Mesh>,
        #[serde(default)]
        ///An array of nodes.
        pub nodes: Vec<crate::generated::gltf::Node>,
        #[serde(default)]
        ///An array of samplers.  A sampler contains properties for texture filtering and wrapping modes.
        pub samplers: Vec<crate::generated::gltf::Sampler>,
        #[serde(default)]
        ///The index of the default scene.  This property **MUST NOT** be defined, when `scenes` is undefined.
        pub scene: Option<i64>,
        #[serde(default)]
        ///An array of scenes.
        pub scenes: Vec<crate::generated::gltf::Scene>,
        #[serde(default)]
        ///An array of skins.  A skin is defined by joints and matrices.
        pub skins: Vec<crate::generated::gltf::Skin>,
        #[serde(default)]
        ///An array of textures.
        pub textures: Vec<crate::generated::gltf::Texture>,
    }
    impl crate::GltfObject for Gltf {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf::Gltf;
mod accessor {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Type {
        #[serde(rename = "SCALAR")]
        Scalar,
        #[serde(rename = "VEC2")]
        Vec2,
        #[serde(rename = "VEC3")]
        Vec3,
        #[serde(rename = "VEC4")]
        Vec4,
        #[serde(rename = "MAT2")]
        Mat2,
        #[serde(rename = "MAT3")]
        Mat3,
        #[serde(rename = "MAT4")]
        Mat4,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///A typed view into a buffer view that contains raw binary data.
    pub struct Accessor {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The index of the buffer view. When undefined, the accessor **MUST** be initialized with zeros; `sparse` property or extensions **MAY** override zeros with actual values.
        pub buffer_view: Option<i64>,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        ///The offset relative to the start of the buffer view in bytes.  This **MUST** be a multiple of the size of the component datatype. This property **MUST NOT** be defined when `bufferView` is undefined.
        pub byte_offset: Option<i64>,
        #[serde(rename = "componentType")]
        ///The datatype of the accessor's components.  UNSIGNED_INT type **MUST NOT** be used for any accessor that is not referenced by `mesh.primitive.indices`.
        pub component_type: i64,
        ///The number of elements referenced by this accessor, not to be confused with the number of bytes or number of components.
        pub count: i64,
        #[serde(default)]
        /**Maximum value of each component in this accessor.  Array elements **MUST** be treated as having the same data type as accessor's `componentType`. Both `min` and `max` arrays have the same length.  The length is determined by the value of the `type` property; it can be 1, 2, 3, 4, 9, or 16.

`normalized` property has no effect on array values: they always correspond to the actual values stored in the buffer. When the accessor is sparse, this property **MUST** contain maximum values of accessor data with sparse substitution applied.*/
        pub max: Vec<f64>,
        #[serde(default)]
        /**Minimum value of each component in this accessor.  Array elements **MUST** be treated as having the same data type as accessor's `componentType`. Both `min` and `max` arrays have the same length.  The length is determined by the value of the `type` property; it can be 1, 2, 3, 4, 9, or 16.

`normalized` property has no effect on array values: they always correspond to the actual values stored in the buffer. When the accessor is sparse, this property **MUST** contain minimum values of accessor data with sparse substitution applied.*/
        pub min: Vec<f64>,
        #[serde(default)]
        ///Specifies whether integer data values are normalized (`true`) to [0, 1] (for unsigned types) or to [-1, 1] (for signed types) when they are accessed. This property **MUST NOT** be set to `true` for accessors with `FLOAT` or `UNSIGNED_INT` component type.
        pub normalized: Option<bool>,
        #[serde(default)]
        ///Sparse storage of elements that deviate from their initialization value.
        pub sparse: Option<crate::generated::gltf::AccessorSparse>,
        #[serde(rename = "type")]
        ///Specifies if the accessor's elements are scalars, vectors, or matrices.
        pub ty: Type,
    }
    impl crate::GltfObject for Accessor {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use accessor::Accessor;
mod scene {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///The root nodes of a scene.
    pub struct Scene {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
        pub name: Option<String>,
        #[serde(default)]
        ///The indices of each root node.
        pub nodes: Vec<i64>,
    }
    impl crate::GltfObject for Scene {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use scene::Scene;
