mod gltf {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "The root object for a glTF asset."]
    pub struct glTF {
        #[serde(rename = "extensionsRequired")]
        #[doc = "Names of glTF extensions required to properly load this asset."]
        extensions_required: Vec<String>,
        #[serde(rename = "materials")]
        #[doc = "An array of materials."]
        materials: Vec<super::material::Material>,
        #[serde(rename = "cameras")]
        #[doc = "An array of cameras."]
        cameras: Vec<super::camera::Camera>,
        #[serde(rename = "accessors")]
        #[doc = "An array of accessors."]
        accessors: Vec<super::accessor::Accessor>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "images")]
        #[doc = "An array of images."]
        images: Vec<super::image::Image>,
        #[serde(rename = "nodes")]
        #[doc = "An array of nodes."]
        nodes: Vec<super::node::Node>,
        #[serde(rename = "asset")]
        #[doc = "Metadata about the glTF asset."]
        asset: super::asset::Asset,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "bufferViews")]
        #[doc = "An array of bufferViews."]
        buffer_views: Vec<super::bufferview::BufferView>,
        #[serde(rename = "skins")]
        #[doc = "An array of skins."]
        skins: Vec<super::skin::Skin>,
        #[serde(rename = "extensionsUsed")]
        #[doc = "Names of glTF extensions used in this asset."]
        extensions_used: Vec<String>,
        #[serde(rename = "animations")]
        #[doc = "An array of keyframe animations."]
        animations: Vec<super::animation::Animation>,
        #[serde(rename = "textures")]
        #[doc = "An array of textures."]
        textures: Vec<super::texture::Texture>,
        #[serde(rename = "scenes")]
        #[doc = "An array of scenes."]
        scenes: Vec<super::scene::Scene>,
        #[serde(rename = "samplers")]
        #[doc = "An array of samplers."]
        samplers: Vec<super::sampler::Sampler>,
        #[serde(rename = "buffers")]
        #[doc = "An array of buffers."]
        buffers: Vec<super::buffer::Buffer>,
        #[serde(rename = "scene")]
        #[doc = "The index of the default scene."]
        scene: Option<i64>,
        #[serde(rename = "meshes")]
        #[doc = "An array of meshes."]
        meshes: Vec<super::mesh::Mesh>,
    }
}
mod mesh {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A set of primitives to be rendered.  Its global transform is defined by a node that references it."]
    pub struct Mesh {
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "primitives")]
        #[doc = "An array of primitives, each defining geometry to be rendered."]
        primitives: Vec<super::meshprimitive::MeshPrimitive>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "weights")]
        #[doc = "Array of weights to be applied to the morph targets. The number of array elements **MUST** match the number of morph targets."]
        weights: Vec<f64>,
    }
}
mod meshprimitive {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Geometry to be rendered with the given material."]
    pub struct MeshPrimitive {
        #[serde(rename = "mode")]
        #[doc = "The topology type of primitives to render."]
        mode: serde_json::Value,
        #[serde(rename = "indices")]
        #[doc = "The index of the accessor that contains the vertex indices."]
        indices: Option<i64>,
        #[serde(rename = "attributes")]
        #[doc = "A plain JSON object, where each key corresponds to a mesh attribute semantic and each value is the index of the accessor containing attribute's data."]
        attributes: Map<String, Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "material")]
        #[doc = "The index of the material to apply to this primitive when rendering."]
        material: Option<i64>,
        #[serde(rename = "targets")]
        #[doc = "An array of morph targets."]
        targets: Vec<Map<String, Value>>,
    }
}
mod buffer {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A buffer points to binary geometry, animation, or skins."]
    pub struct Buffer {
        #[serde(rename = "byteLength")]
        #[doc = "The length of the buffer in bytes."]
        byte_length: i64,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "uri")]
        #[doc = "The URI (or IRI) of the buffer."]
        uri: Option<String>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
    }
}
mod sampler {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Texture sampler properties for filtering and wrapping modes."]
    pub struct Sampler {
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "magFilter")]
        #[doc = "Magnification filter."]
        mag_filter: Option<serde_json::Value>,
        #[serde(rename = "minFilter")]
        #[doc = "Minification filter."]
        min_filter: Option<serde_json::Value>,
        #[serde(rename = "wrapS")]
        #[doc = "S (U) wrapping mode."]
        wrap_s: serde_json::Value,
        #[serde(rename = "wrapT")]
        #[doc = "T (V) wrapping mode."]
        wrap_t: serde_json::Value,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
    }
}
mod scene {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "The root nodes of a scene."]
    pub struct Scene {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "nodes")]
        #[doc = "The indices of each root node."]
        nodes: Vec<i64>,
    }
}
mod texture {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A texture and its sampler."]
    pub struct Texture {
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "sampler")]
        #[doc = "The index of the sampler used by this texture. When undefined, a sampler with repeat wrapping and auto filtering **SHOULD** be used."]
        sampler: Option<i64>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "source")]
        #[doc = "The index of the image used by this texture. When undefined, an extension or other mechanism **SHOULD** supply an alternate texture source, otherwise behavior is undefined."]
        source: Option<i64>,
    }
}
mod animation {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A keyframe animation."]
    pub struct Animation {
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "channels")]
        #[doc = "An array of animation channels. An animation channel combines an animation sampler with a target property being animated. Different channels of the same animation **MUST NOT** have the same targets."]
        channels: Vec<super::animationchannel::AnimationChannel>,
        #[serde(rename = "samplers")]
        #[doc = "An array of animation samplers. An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm."]
        samplers: Vec<super::animationsampler::AnimationSampler>,
    }
}
mod animationsampler {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    #[derive(Default)]
    enum Interpolation {
        #[serde(rename = "LINEAR")]
        #[default]
        Linear,
        #[serde(rename = "STEP")]
        Step,
        #[serde(rename = "CUBICSPLINE")]
        Cubicspline,
    }
    #[derive(Serialize, Deserialize)]
    #[doc = "An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm."]
    pub struct AnimationSampler {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "input")]
        #[doc = "The index of an accessor containing keyframe timestamps."]
        input: i64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "interpolation")]
        #[serde(default)]
        #[doc = "Interpolation algorithm."]
        interpolation: Interpolation,
        #[serde(rename = "output")]
        #[doc = "The index of an accessor, containing keyframe output values."]
        output: i64,
    }
}
mod animationchannel {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "An animation channel combines an animation sampler with a target property being animated."]
    pub struct AnimationChannel {
        #[serde(rename = "target")]
        #[doc = "The descriptor of the animated property."]
        target: super::animationchanneltarget::AnimationChannelTarget,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "sampler")]
        #[doc = "The index of a sampler in this animation used to compute the value for the target."]
        sampler: i64,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
    }
}
mod animationchanneltarget {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum Path {
        #[serde(rename = "translation")]
        Translation,
        #[serde(rename = "rotation")]
        Rotation,
        #[serde(rename = "scale")]
        Scale,
        #[serde(rename = "weights")]
        Weights,
    }
    #[derive(Serialize, Deserialize)]
    #[doc = "The descriptor of the animated property."]
    pub struct AnimationChannelTarget {
        #[serde(rename = "path")]
        #[doc = "The name of the node's TRS property to animate, or the `\"weights\"` of the Morph Targets it instantiates. For the `\"translation\"` property, the values that are provided by the sampler are the translation along the X, Y, and Z axes. For the `\"rotation\"` property, the values are a quaternion in the order (x, y, z, w), where w is the scalar. For the `\"scale\"` property, the values are the scaling factors along the X, Y, and Z axes."]
        path: Path,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "node")]
        #[doc = "The index of the node to animate. When undefined, the animated object **MAY** be defined by an extension."]
        node: Option<i64>,
    }
}
mod skin {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Joints and matrices defining a skin."]
    pub struct Skin {
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "joints")]
        #[doc = "Indices of skeleton nodes, used as joints in this skin."]
        joints: Vec<i64>,
        #[serde(rename = "skeleton")]
        #[doc = "The index of the node used as a skeleton root."]
        skeleton: Option<i64>,
        #[serde(rename = "inverseBindMatrices")]
        #[doc = "The index of the accessor containing the floating-point 4x4 inverse-bind matrices."]
        inverse_bind_matrices: Option<i64>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
    }
}
mod bufferview {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A view into a buffer generally representing a subset of the buffer."]
    pub struct BufferView {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "byteStride")]
        #[doc = "The stride, in bytes."]
        byte_stride: Option<i64>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        #[doc = "The offset into the buffer in bytes."]
        byte_offset: i64,
        #[serde(rename = "byteLength")]
        #[doc = "The length of the bufferView in bytes."]
        byte_length: i64,
        #[serde(rename = "target")]
        #[doc = "The hint representing the intended GPU buffer type to use with this buffer view."]
        target: Option<serde_json::Value>,
        #[serde(rename = "buffer")]
        #[doc = "The index of the buffer."]
        buffer: i64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
    }
}
mod asset {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Metadata about the glTF asset."]
    pub struct Asset {
        #[serde(rename = "generator")]
        #[doc = "Tool that generated this glTF model.  Useful for debugging."]
        generator: Option<String>,
        #[serde(rename = "version")]
        #[doc = "The glTF version in the form of `<major>.<minor>` that this asset targets."]
        version: String,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "copyright")]
        #[doc = "A copyright message suitable for display to credit the content creator."]
        copyright: Option<String>,
        #[serde(rename = "minVersion")]
        #[doc = "The minimum glTF version in the form of `<major>.<minor>` that this asset targets. This property **MUST NOT** be greater than the asset version."]
        min_version: Option<String>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
    }
}
mod node {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present."]
    pub struct Node {
        #[serde(rename = "translation")]
        #[doc = "The node's translation along the x, y, and z axes."]
        translation: [f64; 3usize],
        #[serde(rename = "weights")]
        #[doc = "The weights of the instantiated morph target. The number of array elements **MUST** match the number of morph targets of the referenced mesh. When defined, `mesh` **MUST** also be defined."]
        weights: Vec<f64>,
        #[serde(rename = "children")]
        #[doc = "The indices of this node's children."]
        children: Vec<i64>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "matrix")]
        #[doc = "A floating-point 4x4 transformation matrix stored in column-major order."]
        matrix: [f64; 16usize],
        #[serde(rename = "mesh")]
        #[doc = "The index of the mesh in this node."]
        mesh: Option<i64>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "camera")]
        #[doc = "The index of the camera referenced by this node."]
        camera: Option<i64>,
        #[serde(rename = "rotation")]
        #[doc = "The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar."]
        rotation: [f64; 4usize],
        #[serde(rename = "scale")]
        #[doc = "The node's non-uniform scale, given as the scaling factors along the x, y, and z axes."]
        scale: [f64; 3usize],
        #[serde(rename = "skin")]
        #[doc = "The index of the skin referenced by this node."]
        skin: Option<i64>,
    }
}
mod image {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum MimeType {
        #[serde(rename = "image/jpeg")]
        Imagejpeg,
        #[serde(rename = "image/png")]
        Imagepng,
    }
    #[derive(Serialize, Deserialize)]
    #[doc = "Image data used to create a texture. Image **MAY** be referenced by an URI (or IRI) or a buffer view index."]
    pub struct Image {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "bufferView")]
        #[doc = "The index of the bufferView that contains the image. This field **MUST NOT** be defined when `uri` is defined."]
        buffer_view: Option<i64>,
        #[serde(rename = "uri")]
        #[doc = "The URI (or IRI) of the image."]
        uri: Option<String>,
        #[serde(rename = "mimeType")]
        #[doc = "The image's media type. This field **MUST** be defined when `bufferView` is defined."]
        mime_ty: Option<MimeType>,
    }
}
mod accessor {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum Type {
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
    #[derive(Serialize, Deserialize)]
    #[doc = "A typed view into a buffer view that contains raw binary data."]
    pub struct Accessor {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "bufferView")]
        #[doc = "The index of the bufferView."]
        buffer_view: Option<i64>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        #[doc = "The offset relative to the start of the buffer view in bytes."]
        byte_offset: i64,
        #[serde(rename = "max")]
        #[doc = "Maximum value of each component in this accessor."]
        max: Vec<f64>,
        #[serde(rename = "normalized")]
        #[serde(default)]
        #[doc = "Specifies whether integer data values are normalized before usage."]
        normalized: bool,
        #[serde(rename = "count")]
        #[doc = "The number of elements referenced by this accessor."]
        count: i64,
        #[serde(rename = "sparse")]
        #[doc = "Sparse storage of elements that deviate from their initialization value."]
        sparse: Option<super::accessorsparse::AccessorSparse>,
        #[serde(rename = "type")]
        #[doc = "Specifies if the accessor's elements are scalars, vectors, or matrices."]
        ty: Type,
        #[serde(rename = "min")]
        #[doc = "Minimum value of each component in this accessor."]
        min: Vec<f64>,
        #[serde(rename = "componentType")]
        #[doc = "The datatype of the accessor's components."]
        component_ty: serde_json::Value,
    }
}
mod accessorsparse {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Sparse storage of accessor values that deviate from their initialization value."]
    pub struct AccessorSparse {
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "values")]
        #[doc = "An object pointing to a buffer view containing the deviating accessor values."]
        values: super::accessorsparsevalues::AccessorSparseValues,
        #[serde(rename = "count")]
        #[doc = "Number of deviating accessor values stored in the sparse array."]
        count: i64,
        #[serde(rename = "indices")]
        #[doc = "An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase."]
        indices: super::accessorsparseindices::AccessorSparseIndices,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
    }
}
mod accessorsparseindices {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `accessor.sparse.count`. Indices **MUST** strictly increase."]
    pub struct AccessorSparseIndices {
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "componentType")]
        #[doc = "The indices data type."]
        component_ty: serde_json::Value,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        #[doc = "The offset relative to the start of the buffer view in bytes."]
        byte_offset: i64,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "bufferView")]
        #[doc = "The index of the buffer view with sparse indices. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined. The buffer view and the optional `byteOffset` **MUST** be aligned to the `componentType` byte length."]
        buffer_view: i64,
    }
}
mod accessorsparsevalues {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "An object pointing to a buffer view containing the deviating accessor values. The number of elements is equal to `accessor.sparse.count` times number of components. The elements have the same component type as the base accessor. The elements are tightly packed. Data **MUST** be aligned following the same rules as the base accessor."]
    pub struct AccessorSparseValues {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "bufferView")]
        #[doc = "The index of the bufferView with sparse values. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined."]
        buffer_view: i64,
        #[serde(rename = "byteOffset")]
        #[serde(default)]
        #[doc = "The offset relative to the start of the bufferView in bytes."]
        byte_offset: i64,
    }
}
mod camera {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum Type {
        #[serde(rename = "perspective")]
        Perspective,
        #[serde(rename = "orthographic")]
        Orthographic,
    }
    #[derive(Serialize, Deserialize)]
    #[doc = "A camera's projection.  A node **MAY** reference a camera to apply a transform to place the camera in the scene."]
    pub struct Camera {
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "type")]
        #[doc = "Specifies if the camera uses a perspective or orthographic projection."]
        ty: Type,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "perspective")]
        #[doc = "A perspective camera containing properties to create a perspective projection matrix. This property **MUST NOT** be defined when `orthographic` is defined."]
        perspective: Option<super::cameraperspective::CameraPerspective>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "orthographic")]
        #[doc = "An orthographic camera containing properties to create an orthographic projection matrix. This property **MUST NOT** be defined when `perspective` is defined."]
        orthographic: Option<super::cameraorthographic::CameraOrthographic>,
    }
}
mod cameraorthographic {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "An orthographic camera containing properties to create an orthographic projection matrix."]
    pub struct CameraOrthographic {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "ymag")]
        #[doc = "The floating-point vertical magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative."]
        ymag: f64,
        #[serde(rename = "zfar")]
        #[doc = "The floating-point distance to the far clipping plane. This value **MUST NOT** be equal to zero. `zfar` **MUST** be greater than `znear`."]
        zfar: f64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "xmag")]
        #[doc = "The floating-point horizontal magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative."]
        xmag: f64,
        #[serde(rename = "znear")]
        #[doc = "The floating-point distance to the near clipping plane."]
        znear: f64,
    }
}
mod cameraperspective {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A perspective camera containing properties to create a perspective projection matrix."]
    pub struct CameraPerspective {
        #[serde(rename = "yfov")]
        #[doc = "The floating-point vertical field of view in radians. This value **SHOULD** be less than π."]
        yfov: f64,
        #[serde(rename = "zfar")]
        #[doc = "The floating-point distance to the far clipping plane."]
        zfar: Option<f64>,
        #[serde(rename = "znear")]
        #[doc = "The floating-point distance to the near clipping plane."]
        znear: f64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "aspectRatio")]
        #[doc = "The floating-point aspect ratio of the field of view."]
        aspect_ratio: Option<f64>,
    }
}
mod material {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    #[derive(Default)]
    enum AlphaMode {
        #[serde(rename = "OPAQUE")]
        #[default]
        Opaque,
        #[serde(rename = "MASK")]
        Mask,
        #[serde(rename = "BLEND")]
        Blend,
    }
    #[derive(Serialize, Deserialize)]
    #[doc = "The material appearance of a primitive."]
    pub struct Material {
        #[serde(rename = "doubleSided")]
        #[serde(default)]
        #[doc = "Specifies whether the material is double sided."]
        double_sided: bool,
        #[serde(rename = "alphaMode")]
        #[serde(default)]
        #[doc = "The alpha rendering mode of the material."]
        alpha_mode: AlphaMode,
        #[serde(rename = "pbrMetallicRoughness")]
        #[doc = "A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply."]
        pbr_metallic_roughness:
            Option<super::materialpbrmetallicroughness::MaterialPBRMetallicRoughness>,
        #[serde(rename = "emissiveTexture")]
        #[doc = "The emissive texture."]
        emissive_texture: Option<super::textureinfo::TextureInfo>,
        #[serde(rename = "normalTexture")]
        #[doc = "The tangent space normal texture."]
        normal_texture: Option<super::materialnormaltextureinfo::MaterialNormalTextureInfo>,
        #[serde(rename = "occlusionTexture")]
        #[doc = "The occlusion texture."]
        occlusion_texture:
            Option<super::materialocclusiontextureinfo::MaterialOcclusionTextureInfo>,
        #[serde(rename = "name")]
        #[doc = "The user-defined name of this object."]
        name: Option<String>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "emissiveFactor")]
        #[doc = "The factors for the emissive color of the material."]
        emissive_factor: [f64; 3usize],
        #[serde(rename = "alphaCutoff")]
        #[doc = "The alpha cutoff value of the material."]
        alpha_cutoff: f64,
    }
}
mod materialocclusiontextureinfo {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    pub struct MaterialOcclusionTextureInfo {
        #[serde(rename = "texCoord")]
        #[serde(default)]
        #[doc = "The set index of texture's TEXCOORD attribute used for texture coordinate mapping."]
        tex_coord: i64,
        #[serde(rename = "strength")]
        #[doc = "A scalar multiplier controlling the amount of occlusion applied."]
        strength: f64,
        #[serde(rename = "index")]
        #[doc = "The index of the texture."]
        index: i64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
    }
}
mod materialnormaltextureinfo {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    pub struct MaterialNormalTextureInfo {
        #[serde(rename = "index")]
        #[doc = "The index of the texture."]
        index: i64,
        #[serde(rename = "scale")]
        #[doc = "The scalar parameter applied to each normal vector of the normal texture."]
        scale: f64,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "texCoord")]
        #[serde(default)]
        #[doc = "The set index of texture's TEXCOORD attribute used for texture coordinate mapping."]
        tex_coord: i64,
    }
}
mod textureinfo {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "Reference to a texture."]
    pub struct TextureInfo {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "index")]
        #[doc = "The index of the texture."]
        index: i64,
        #[serde(rename = "texCoord")]
        #[serde(default)]
        #[doc = "The set index of texture's TEXCOORD attribute used for texture coordinate mapping."]
        tex_coord: i64,
    }
}
mod materialpbrmetallicroughness {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize)]
    #[doc = "A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology."]
    pub struct MaterialPBRMetallicRoughness {
        #[serde(rename = "extensions")]
        #[doc = "JSON object with extension-specific objects."]
        extensions: Option<Map<String, Value>>,
        #[serde(rename = "extras")]
        #[doc = "Application-specific data."]
        extras: Option<serde_json::Value>,
        #[serde(rename = "baseColorFactor")]
        #[doc = "The factors for the base color of the material."]
        base_color_factor: [f64; 4usize],
        #[serde(rename = "baseColorTexture")]
        #[doc = "The base color texture."]
        base_color_texture: Option<super::textureinfo::TextureInfo>,
        #[serde(rename = "metallicRoughnessTexture")]
        #[doc = "The metallic-roughness texture."]
        metallic_roughness_texture: Option<super::textureinfo::TextureInfo>,
        #[serde(rename = "roughnessFactor")]
        #[doc = "The factor for the roughness of the material."]
        roughness_factor: f64,
        #[serde(rename = "metallicFactor")]
        #[doc = "The factor for the metalness of the material."]
        metallic_factor: f64,
    }
}
