#![allow(clippy::all, unused_imports)]
mod gltf {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///The root object for a glTF asset.
    pub struct Gltf {
        #[serde(default)]
        ///An array of skins.
        pub skins: Vec<crate::generated::gltf::Skin>,
        #[serde(default)]
        ///An array of textures.
        pub textures: Vec<crate::generated::gltf::Texture>,
        #[serde(rename = "extensionsUsed")]
        #[serde(default)]
        ///Names of glTF extensions used in this asset.
        pub extensions_used: Vec<String>,
        #[serde(default)]
        ///An array of cameras.
        pub cameras: Vec<crate::generated::gltf::Camera>,
        #[serde(default)]
        ///An array of materials.
        pub materials: Vec<crate::generated::gltf::Material>,
        #[serde(default)]
        ///An array of keyframe animations.
        pub animations: Vec<crate::generated::gltf::Animation>,
        #[serde(rename = "extensionsRequired")]
        #[serde(default)]
        ///Names of glTF extensions required to properly load this asset.
        pub extensions_required: Vec<String>,
        #[serde(default)]
        ///An array of accessors.
        pub accessors: Vec<crate::generated::gltf::Accessor>,
        #[serde(default)]
        ///An array of images.
        pub images: Vec<crate::generated::gltf::Image>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the default scene.
        pub scene: Option<i64>,
        ///Metadata about the glTF asset.
        pub asset: crate::generated::gltf::Asset,
        #[serde(default)]
        ///An array of meshes.
        pub meshes: Vec<crate::generated::gltf::Mesh>,
        #[serde(default)]
        ///An array of nodes.
        pub nodes: Vec<crate::generated::gltf::Node>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "bufferViews")]
        #[serde(default)]
        ///An array of bufferViews.
        pub buffer_views: Vec<crate::generated::gltf::BufferView>,
        #[serde(default)]
        ///An array of buffers.
        pub buffers: Vec<crate::generated::gltf::Buffer>,
        #[serde(default)]
        ///An array of samplers.
        pub samplers: Vec<crate::generated::gltf::Sampler>,
        #[serde(default)]
        ///An array of scenes.
        pub scenes: Vec<crate::generated::gltf::Scene>,
    }
    impl crate::GltfObject for Gltf {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use gltf::Gltf;
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
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///The index of the sampler used by this texture. When undefined, a sampler with repeat wrapping and auto filtering **SHOULD** be used.
        pub sampler: Option<i64>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
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
mod skin {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Joints and matrices defining a skin.
    pub struct Skin {
        #[serde(default)]
        ///The index of the node used as a skeleton root.
        pub skeleton: Option<i64>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        ///Indices of skeleton nodes, used as joints in this skin.
        pub joints: Vec<i64>,
        #[serde(rename = "inverseBindMatrices")]
        #[serde(default)]
        ///The index of the accessor containing the floating-point 4x4 inverse-bind matrices.
        pub inverse_bind_matrices: Option<i64>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Skin {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use skin::Skin;
mod scene {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///The root nodes of a scene.
    pub struct Scene {
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
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
mod sampler {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Texture sampler properties for filtering and wrapping modes.
    pub struct Sampler {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "minFilter")]
        #[serde(default)]
        ///Minification filter.
        pub min_filter: Option<i64>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "wrapS")]
        #[serde(default = "get_default_wrap_s")]
        ///S (U) wrapping mode.
        pub wrap_s: i64,
        #[serde(rename = "wrapT")]
        #[serde(default = "get_default_wrap_t")]
        ///T (V) wrapping mode.
        pub wrap_t: i64,
        #[serde(rename = "magFilter")]
        #[serde(default)]
        ///Magnification filter.
        pub mag_filter: Option<i64>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Sampler {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_wrap_s() -> i64 {
        10497i64
    }
    fn get_default_wrap_t() -> i64 {
        10497i64
    }
}
pub use sampler::Sampler;
mod node {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present.
    pub struct Node {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The index of the camera referenced by this node.
        pub camera: Option<i64>,
        #[serde(default = "get_default_translation")]
        ///The node's translation along the x, y, and z axes.
        pub translation: [f64; 3usize],
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///The weights of the instantiated morph target. The number of array elements **MUST** match the number of morph targets of the referenced mesh. When defined, `mesh` **MUST** also be defined.
        pub weights: Vec<f64>,
        #[serde(default)]
        ///The index of the mesh in this node.
        pub mesh: Option<i64>,
        #[serde(default = "get_default_matrix")]
        ///A floating-point 4x4 transformation matrix stored in column-major order.
        pub matrix: [f64; 16usize],
        #[serde(default = "get_default_scale")]
        ///The node's non-uniform scale, given as the scaling factors along the x, y, and z axes.
        pub scale: [f64; 3usize],
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default = "get_default_rotation")]
        ///The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
        pub rotation: [f64; 4usize],
        #[serde(default)]
        ///The indices of this node's children.
        pub children: Vec<i64>,
        #[serde(default)]
        ///The index of the skin referenced by this node.
        pub skin: Option<i64>,
    }
    impl crate::GltfObject for Node {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_translation() -> [f64; 3usize] {
        [0f64, 0f64, 0f64]
    }
    fn get_default_matrix() -> [f64; 16usize] {
        [
            1f64,
            0f64,
            0f64,
            0f64,
            0f64,
            1f64,
            0f64,
            0f64,
            0f64,
            0f64,
            1f64,
            0f64,
            0f64,
            0f64,
            0f64,
            1f64,
        ]
    }
    fn get_default_scale() -> [f64; 3usize] {
        [1f64, 1f64, 1f64]
    }
    fn get_default_rotation() -> [f64; 4usize] {
        [0f64, 0f64, 0f64, 1f64]
    }
}
pub use node::Node;
mod mesh {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A set of primitives to be rendered.  Its global transform is defined by a node that references it.
    pub struct Mesh {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///An array of primitives, each defining geometry to be rendered.
        pub primitives: Vec<crate::generated::gltf::MeshPrimitive>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
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
        ///The index of the material to apply to this primitive when rendering.
        pub material: Option<i64>,
        #[serde(default = "get_default_mode")]
        ///The topology type of primitives to render.
        pub mode: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///An array of morph targets.
        pub targets: Vec<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///A plain JSON object, where each key corresponds to a mesh attribute semantic and each value is the index of the accessor containing attribute's data.
        pub attributes: Map<String, Value>,
        #[serde(default)]
        ///The index of the accessor that contains the vertex indices.
        pub indices: Option<i64>,
    }
    impl crate::GltfObject for MeshPrimitive {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_mode() -> i64 {
        4i64
    }
}
pub use mesh_primitive::MeshPrimitive;
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
        #[serde(rename = "alphaMode")]
        #[serde(default = "get_default_alpha_mode")]
        ///The alpha rendering mode of the material.
        pub alpha_mode: AlphaMode,
        #[serde(rename = "emissiveFactor")]
        #[serde(default = "get_default_emissive_factor")]
        ///The factors for the emissive color of the material.
        pub emissive_factor: [f64; 3usize],
        #[serde(rename = "pbrMetallicRoughness")]
        #[serde(default)]
        ///A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply.
        pub pbr_metallic_roughness: Option<
            crate::generated::gltf::MaterialPbrMetallicRoughness,
        >,
        #[serde(rename = "normalTexture")]
        #[serde(default)]
        ///The tangent space normal texture.
        pub normal_texture: Option<crate::generated::gltf::MaterialNormalTextureInfo>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "doubleSided")]
        #[serde(default = "get_default_double_sided")]
        ///Specifies whether the material is double sided.
        pub double_sided: bool,
        #[serde(rename = "occlusionTexture")]
        #[serde(default)]
        ///The occlusion texture.
        pub occlusion_texture: Option<
            crate::generated::gltf::MaterialOcclusionTextureInfo,
        >,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "emissiveTexture")]
        #[serde(default)]
        ///The emissive texture.
        pub emissive_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "alphaCutoff")]
        #[serde(default = "get_default_alpha_cutoff")]
        ///The alpha cutoff value of the material.
        pub alpha_cutoff: f64,
    }
    impl crate::GltfObject for Material {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_alpha_mode() -> AlphaMode {
        AlphaMode::default()
    }
    fn get_default_emissive_factor() -> [f64; 3usize] {
        [0f64, 0f64, 0f64]
    }
    fn get_default_double_sided() -> bool {
        false
    }
    fn get_default_alpha_cutoff() -> f64 {
        0.5f64
    }
}
pub use material::Material;
mod material_pbr_metallic_roughness {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology.
    pub struct MaterialPbrMetallicRoughness {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "metallicFactor")]
        #[serde(default = "get_default_metallic_factor")]
        ///The factor for the metalness of the material.
        pub metallic_factor: f64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "baseColorFactor")]
        #[serde(default = "get_default_base_color_factor")]
        ///The factors for the base color of the material.
        pub base_color_factor: [f64; 4usize],
        #[serde(rename = "roughnessFactor")]
        #[serde(default = "get_default_roughness_factor")]
        ///The factor for the roughness of the material.
        pub roughness_factor: f64,
        #[serde(rename = "baseColorTexture")]
        #[serde(default)]
        ///The base color texture.
        pub base_color_texture: Option<crate::generated::gltf::TextureInfo>,
        #[serde(rename = "metallicRoughnessTexture")]
        #[serde(default)]
        ///The metallic-roughness texture.
        pub metallic_roughness_texture: Option<crate::generated::gltf::TextureInfo>,
    }
    impl crate::GltfObject for MaterialPbrMetallicRoughness {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_metallic_factor() -> f64 {
        1f64
    }
    fn get_default_base_color_factor() -> [f64; 4usize] {
        [1f64, 1f64, 1f64, 1f64]
    }
    fn get_default_roughness_factor() -> f64 {
        1f64
    }
}
pub use material_pbr_metallic_roughness::MaterialPbrMetallicRoughness;
mod material_occlusion_texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MaterialOcclusionTextureInfo {
        #[serde(default = "get_default_strength")]
        ///A scalar multiplier controlling the amount of occlusion applied.
        pub strength: f64,
        ///The index of the texture.
        pub index: i64,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "texCoord")]
        #[serde(default = "get_default_tex_coord")]
        ///The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
        pub tex_coord: i64,
    }
    impl crate::GltfObject for MaterialOcclusionTextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_strength() -> f64 {
        1f64
    }
    fn get_default_tex_coord() -> i64 {
        0i64
    }
}
pub use material_occlusion_texture_info::MaterialOcclusionTextureInfo;
mod material_normal_texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MaterialNormalTextureInfo {
        #[serde(default = "get_default_scale")]
        ///The scalar parameter applied to each normal vector of the normal texture.
        pub scale: f64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of the texture.
        pub index: i64,
        #[serde(rename = "texCoord")]
        #[serde(default = "get_default_tex_coord")]
        ///The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
        pub tex_coord: i64,
    }
    impl crate::GltfObject for MaterialNormalTextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_scale() -> f64 {
        1f64
    }
    fn get_default_tex_coord() -> i64 {
        0i64
    }
}
pub use material_normal_texture_info::MaterialNormalTextureInfo;
mod texture_info {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Reference to a texture.
    pub struct TextureInfo {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "texCoord")]
        #[serde(default = "get_default_tex_coord")]
        ///The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
        pub tex_coord: i64,
        ///The index of the texture.
        pub index: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for TextureInfo {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_tex_coord() -> i64 {
        0i64
    }
}
pub use texture_info::TextureInfo;
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
        #[serde(rename = "mimeType")]
        #[serde(default)]
        ///The image's media type. This field **MUST** be defined when `bufferView` is defined.
        pub mime_ty: Option<MimeType>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The index of the bufferView that contains the image. This field **MUST NOT** be defined when `uri` is defined.
        pub buffer_view: Option<i64>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The URI (or IRI) of the image.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for Image {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use image::Image;
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
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///An orthographic camera containing properties to create an orthographic projection matrix. This property **MUST NOT** be defined when `perspective` is defined.
        pub orthographic: Option<crate::generated::gltf::CameraOrthographic>,
        #[serde(default)]
        ///A perspective camera containing properties to create a perspective projection matrix. This property **MUST NOT** be defined when `orthographic` is defined.
        pub perspective: Option<crate::generated::gltf::CameraPerspective>,
        #[serde(rename = "type")]
        ///Specifies if the camera uses a perspective or orthographic projection.
        pub ty: Type,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Camera {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera::Camera;
mod camera_perspective {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A perspective camera containing properties to create a perspective projection matrix.
    pub struct CameraPerspective {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        ///The floating-point distance to the near clipping plane.
        pub znear: f64,
        #[serde(rename = "aspectRatio")]
        #[serde(default)]
        ///The floating-point aspect ratio of the field of view.
        pub aspect_ratio: Option<f64>,
        ///The floating-point vertical field of view in radians. This value **SHOULD** be less than π.
        pub yfov: f64,
        #[serde(default)]
        ///The floating-point distance to the far clipping plane.
        pub zfar: Option<f64>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
    }
    impl crate::GltfObject for CameraPerspective {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera_perspective::CameraPerspective;
mod camera_orthographic {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An orthographic camera containing properties to create an orthographic projection matrix.
    pub struct CameraOrthographic {
        ///The floating-point distance to the far clipping plane. This value **MUST NOT** be equal to zero. `zfar` **MUST** be greater than `znear`.
        pub zfar: f64,
        ///The floating-point distance to the near clipping plane.
        pub znear: f64,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        ///The floating-point vertical magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
        pub ymag: f64,
        ///The floating-point horizontal magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
        pub xmag: f64,
    }
    impl crate::GltfObject for CameraOrthographic {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use camera_orthographic::CameraOrthographic;
mod buffer {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A buffer points to binary geometry, animation, or skins.
    pub struct Buffer {
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///The URI (or IRI) of the buffer.
        pub uri: Option<String>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "byteLength")]
        ///The length of the buffer in bytes.
        pub byte_length: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Buffer {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use buffer::Buffer;
mod buffer_view {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///A view into a buffer generally representing a subset of the buffer.
    pub struct BufferView {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of the buffer.
        pub buffer: i64,
        #[serde(rename = "byteLength")]
        ///The length of the bufferView in bytes.
        pub byte_length: i64,
        #[serde(rename = "byteStride")]
        #[serde(default)]
        ///The stride, in bytes.
        pub byte_stride: Option<i64>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "byteOffset")]
        #[serde(default = "get_default_byte_offset")]
        ///The offset into the buffer in bytes.
        pub byte_offset: i64,
        #[serde(default)]
        ///The hint representing the intended GPU buffer type to use with this buffer view.
        pub target: Option<i64>,
    }
    impl crate::GltfObject for BufferView {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_byte_offset() -> i64 {
        0i64
    }
}
pub use buffer_view::BufferView;
mod asset {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Metadata about the glTF asset.
    pub struct Asset {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///A copyright message suitable for display to credit the content creator.
        pub copyright: Option<String>,
        #[serde(default)]
        ///Tool that generated this glTF model.  Useful for debugging.
        pub generator: Option<String>,
        ///The glTF version in the form of `<major>.<minor>` that this asset targets.
        pub version: String,
        #[serde(rename = "minVersion")]
        #[serde(default)]
        ///The minimum glTF version in the form of `<major>.<minor>` that this asset targets. This property **MUST NOT** be greater than the asset version.
        pub min_version: Option<String>,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for Asset {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use asset::Asset;
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
        ///An array of animation samplers. An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
        pub samplers: Vec<crate::generated::gltf::AnimationSampler>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        ///An array of animation channels. An animation channel combines an animation sampler with a target property being animated. Different channels of the same animation **MUST NOT** have the same targets.
        pub channels: Vec<crate::generated::gltf::AnimationChannel>,
    }
    impl crate::GltfObject for Animation {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation::Animation;
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
        ///The index of an accessor containing keyframe timestamps.
        pub input: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default = "get_default_interpolation")]
        ///Interpolation algorithm.
        pub interpolation: Interpolation,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///The index of an accessor, containing keyframe output values.
        pub output: i64,
    }
    impl crate::GltfObject for AnimationSampler {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_interpolation() -> Interpolation {
        Interpolation::default()
    }
}
pub use animation_sampler::AnimationSampler;
mod animation_channel {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An animation channel combines an animation sampler with a target property being animated.
    pub struct AnimationChannel {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        ///The index of a sampler in this animation used to compute the value for the target.
        pub sampler: i64,
        ///The descriptor of the animated property.
        pub target: crate::generated::gltf::AnimationChannelTarget,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
    }
    impl crate::GltfObject for AnimationChannel {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use animation_channel::AnimationChannel;
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
        ///The index of the node to animate. When undefined, the animated object **MAY** be defined by an extension.
        pub node: Option<i64>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
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
        ///Minimum value of each component in this accessor.
        pub min: Vec<f64>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The index of the bufferView.
        pub buffer_view: Option<i64>,
        #[serde(rename = "byteOffset")]
        #[serde(default = "get_default_byte_offset")]
        ///The offset relative to the start of the buffer view in bytes.
        pub byte_offset: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(rename = "componentType")]
        ///The datatype of the accessor's components.
        pub component_ty: i64,
        ///The number of elements referenced by this accessor.
        pub count: i64,
        #[serde(default)]
        ///Sparse storage of elements that deviate from their initialization value.
        pub sparse: Option<crate::generated::gltf::AccessorSparse>,
        #[serde(rename = "type")]
        ///Specifies if the accessor's elements are scalars, vectors, or matrices.
        pub ty: Type,
        #[serde(default)]
        ///Maximum value of each component in this accessor.
        pub max: Vec<f64>,
        #[serde(default = "get_default_normalized")]
        ///Specifies whether integer data values are normalized before usage.
        pub normalized: bool,
    }
    impl crate::GltfObject for Accessor {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_byte_offset() -> i64 {
        0i64
    }
    fn get_default_normalized() -> bool {
        false
    }
}
pub use accessor::Accessor;
mod accessor_sparse {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Sparse storage of accessor values that deviate from their initialization value.
    pub struct AccessorSparse {
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        ///An object pointing to a buffer view containing the deviating accessor values.
        pub values: crate::generated::gltf::AccessorSparseValues,
        ///An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase.
        pub indices: crate::generated::gltf::AccessorSparseIndices,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        ///Number of deviating accessor values stored in the sparse array.
        pub count: i64,
    }
    impl crate::GltfObject for AccessorSparse {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use accessor_sparse::AccessorSparse;
mod accessor_sparse_values {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An object pointing to a buffer view containing the deviating accessor values. The number of elements is equal to `accessor.sparse.count` times number of components. The elements have the same component type as the base accessor. The elements are tightly packed. Data **MUST** be aligned following the same rules as the base accessor.
    pub struct AccessorSparseValues {
        #[serde(rename = "byteOffset")]
        #[serde(default = "get_default_byte_offset")]
        ///The offset relative to the start of the bufferView in bytes.
        pub byte_offset: i64,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "bufferView")]
        ///The index of the bufferView with sparse values. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined.
        pub buffer_view: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for AccessorSparseValues {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_byte_offset() -> i64 {
        0i64
    }
}
pub use accessor_sparse_values::AccessorSparseValues;
mod accessor_sparse_indices {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `accessor.sparse.count`. Indices **MUST** strictly increase.
    pub struct AccessorSparseIndices {
        #[serde(rename = "byteOffset")]
        #[serde(default = "get_default_byte_offset")]
        ///The offset relative to the start of the buffer view in bytes.
        pub byte_offset: i64,
        #[serde(rename = "bufferView")]
        ///The index of the buffer view with sparse indices. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined. The buffer view and the optional `byteOffset` **MUST** be aligned to the `componentType` byte length.
        pub buffer_view: i64,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "componentType")]
        ///The indices data type.
        pub component_ty: i64,
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
    }
    impl crate::GltfObject for AccessorSparseIndices {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
    fn get_default_byte_offset() -> i64 {
        0i64
    }
}
pub use accessor_sparse_indices::AccessorSparseIndices;
