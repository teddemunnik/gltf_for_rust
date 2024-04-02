use serde_json::{Map, Value};
use std::error::Error;
use thiserror::Error;

mod generated;

#[cfg(test)]
mod test;
mod import;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ParseExtensionError {
    #[error("The extension was not found on this object.")]
    NotFound,
    #[error("The extension could not be parsed successfully: {inner}")]
    FailedToParse { inner: Box<dyn Error> },
}

/// Trait implemented for all
pub trait GltfObject {
    /// Provides a mechanism to retrieve the extensions for this object
    fn extensions(&self) -> &Option<Map<String, Value>>;

    /// Parses an extension of a specific type out of this Gltf object.
    fn parse_extension<T: GltfExtension>(&self) -> Result<T, ParseExtensionError> {
        let extension = self
            .extensions()
            .as_ref()
            .and_then(|extensions| extensions.get(T::extension_name()));
        if let Some(extension) = extension {
            serde_json::from_value::<T>(extension.clone())
                .map_err(|e| ParseExtensionError::FailedToParse { inner: Box::new(e) })
        } else {
            Err(ParseExtensionError::NotFound)
        }
    }
}

/// Trait implemented for all GLTF extension structures.
pub trait GltfExtension: Sized + for<'a> serde::Deserialize<'a> {
    /// The name of the extension that provides the extension structure implementing this trait.
    /// This will be the key used to look up this extension in an object extension table.
    fn extension_name() -> &'static str;
}
