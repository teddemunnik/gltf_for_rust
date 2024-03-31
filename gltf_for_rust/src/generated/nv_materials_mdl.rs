#![allow(clippy::all, unused_imports)]
pub mod gltf {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables using MDL materials.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "bsdfMeasurements")]
            #[serde(default)]
            ///The list of all BSDF measurements.
            pub bsdf_measurements: Vec<
                crate::generated::nv_materials_mdl::BsdfMeasurement,
            >,
            #[serde(rename = "functionCalls")]
            #[serde(default)]
            ///The list of all function calls.
            pub function_calls: Vec<crate::generated::nv_materials_mdl::FunctionCall>,
            #[serde(default)]
            ///The list of all MDL modules.
            pub modules: Vec<crate::generated::nv_materials_mdl::Module>,
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
            "NV_materials_mdl"
        }
    }
}
pub mod material {
    mod extension {
        use serde::{Serialize, Deserialize};
        use serde_json::{Map, Value};
        #[derive(Serialize, Deserialize, Debug)]
        ///glTF extension that enables using MDL materials.
        pub struct Extension {
            #[serde(default)]
            ///JSON object with extension-specific objects.
            pub extensions: Option<Map<String, Value>>,
            #[serde(default)]
            ///Application-specific data.
            pub extras: Option<serde_json::Value>,
            #[serde(rename = "functionCall")]
            ///The index of the MDL function call. The reference function call **MUST** represent the entry point to a function call graph and have the return type `material`.
            pub function_call: i64,
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
            "NV_materials_mdl"
        }
    }
}
mod module {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum MimeType {
        #[serde(rename = "application/vnd.mdl")]
        ApplicationVndMdl,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///An MDL module.
    pub struct Module {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The ID of the bufferView containing the MDL module.  This field **MUST NOT** be defined if `uri` is defined.
        pub buffer_view: Option<i64>,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        ///The MDL module's media type.  This field **MUST** be defined when `bufferView` is defined.
        pub mime_ty: Option<MimeType>,
        #[serde(rename = "modulePath")]
        #[serde(default)]
        ///Relative path of the module.  This field **MUST** be defined if `bufferView` is defined or `uri` is defined and contains a data-URI, otherwise this field **MUST NOT** be defined.
        pub module_path: Option<String>,
        #[serde(default)]
        ///The URI (or IRI) of the MDL module.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for Module {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use module::Module;
mod function_call {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Function call with its list of arguments. Can represent the entry point into a function call graph or be a node in such a graph.
    pub struct FunctionCall {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(default)]
        ///A list of named value and/or function call arguments.  Multiple arguments with the same name **MUST NOT** exist.
        pub arguments: Vec<crate::generated::nv_materials_mdl::FunctionCallArgument>,
        #[serde(rename = "functionName")]
        ///The unqualified name of the function.
        pub function_name: String,
        #[serde(default)]
        ///The ID of the containing module.  This field **MUST NOT** be defined if a built-in function is specified.
        pub module: Option<i64>,
        #[serde(rename = "type")]
        ///The return type of the function.
        pub ty: crate::generated::nv_materials_mdl::FunctionCallType,
    }
    impl crate::GltfObject for FunctionCall {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use function_call::FunctionCall;
mod function_call_type {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Modifier {
        #[serde(rename = "varying")]
        Varying,
        #[serde(rename = "uniform")]
        Uniform,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///MDL type describing either a built-in or user-defined type, or an array of a built-in or user-defined type.
    pub struct FunctionCallType {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "arraySize")]
        #[serde(default)]
        ///The array size. If this field is defined the type is considered to be a array.
        pub array_size: Option<i64>,
        #[serde(default)]
        ///The name of the type modifier.
        pub modifier: Option<Modifier>,
        #[serde(default)]
        ///The ID of the containing module.  This field **MUST NOT** be defined if a built-in type is specified.
        pub module: Option<i64>,
        #[serde(rename = "typeName")]
        ///The unqualified name of the type.
        pub ty_name: String,
    }
    impl crate::GltfObject for FunctionCallType {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use function_call_type::FunctionCallType;
mod function_call_argument {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    ///Named function call argument. Can be another function call or a constant value.
    pub struct FunctionCallArgument {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(rename = "functionCall")]
        #[serde(default)]
        ///The ID of a function call.  This field **MUST NOT** be defined if `value` is defined.
        pub function_call: Option<i64>,
        #[serde(default)]
        ///The name of the named argument.
        pub name: Option<String>,
        #[serde(rename = "type")]
        #[serde(default)]
        ///The type of the value argument.  This field **MUST** be defined if `value` is defined and **MUST NOT** be defined if `functionCall` is defined.
        pub ty: Option<crate::generated::nv_materials_mdl::FunctionCallType>,
        #[serde(default)]
        ///The literal value of the value argument.  This field **MUST NOT** be defined if `functionCall` is defined.
        pub value: Option<serde_json::Value>,
    }
    impl crate::GltfObject for FunctionCallArgument {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use function_call_argument::FunctionCallArgument;
mod bsdf_measurement {
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};
    #[derive(Serialize, Deserialize, Debug)]
    pub enum MimeType {
        #[serde(rename = "application/vnd.mdl-mbsdf")]
        ApplicationVndMdlMbsdf,
    }
    #[derive(Serialize, Deserialize, Debug)]
    ///A BSDF measurement (MBSDF) as defined in the MDL Language Specification.
    pub struct BsdfMeasurement {
        #[serde(default)]
        ///JSON object with extension-specific objects.
        pub extensions: Option<Map<String, Value>>,
        #[serde(default)]
        ///Application-specific data.
        pub extras: Option<serde_json::Value>,
        #[serde(default)]
        ///The user-defined name of this object.
        pub name: Option<String>,
        #[serde(rename = "bufferView")]
        #[serde(default)]
        ///The ID of the bufferView containing the MBSDF.  This field **MUST NOT** be defined if `uri` is defined.
        pub buffer_view: Option<i64>,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        ///The BSDF measurement's media type.  This field **MUST** be defined when `bufferView` is defined.
        pub mime_ty: Option<MimeType>,
        #[serde(default)]
        ///The URI (or IRI) of the MBSDF.
        pub uri: Option<String>,
    }
    impl crate::GltfObject for BsdfMeasurement {
        fn extensions(&self) -> &Option<Map<String, Value>> {
            &self.extensions
        }
    }
}
pub use bsdf_measurement::BsdfMeasurement;
