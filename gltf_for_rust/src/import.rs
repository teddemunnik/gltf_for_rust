use std::fs::File;
use std::io::BufReader;

use thiserror::Error;

use crate::generated::gltf::Gltf as InnerGltf;
use crate::generated::gltf::Node as InnerNode;
use crate::generated::gltf::Scene as InnerScene;

#[macro_export]
macro_rules! collection_wrapper {
    ($name:ident, $inner_item:ident, $wrapper_item:ident) => {
        /// Iterates over a collection on the gltf root, and returns
        pub struct $name<'a> {
            document: &'a InnerGltf,
            inner: std::iter::Enumerate<std::slice::Iter<'a, $inner_item>>,
        }

        impl<'a> $name<'a> {
            fn new(document: &'a InnerGltf, input: &'a [$inner_item]) -> Self {
                $name {
                    document,
                    inner: input.iter().enumerate(),
                }
            }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = $wrapper_item<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next().map(|(index, inner)| $wrapper_item {
                    document: self.document,
                    index,
                    inner,
                })
            }
        }

        impl<'a> ExactSizeIterator for $name<'a> {}
    };
}

collection_wrapper!(Nodes, InnerNode, Node);
collection_wrapper!(Scenes, InnerScene, Scene);

pub struct NodeChildren<'a> {
    document: &'a InnerGltf,
    inner: std::slice::Iter<'a, i64>,
}

impl<'a> Iterator for NodeChildren<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|index| {
            let index = *index as usize;
            Node {
                document: self.document,
                index,
                inner: &self.document.nodes[index],
            }
        })
    }
}

#[derive(Debug)]
pub struct Gltf {
    inner: InnerGltf,
}

impl Gltf {
    pub fn nodes(&self) -> Nodes {
        Nodes::new(&self.inner, &self.inner.nodes)
    }

    pub fn scenes(&self) -> Scenes {
        Scenes::new(&self.inner, &self.inner.scenes)
    }
}

#[derive(Debug)]
pub struct Node<'a> {
    document: &'a InnerGltf,
    index: usize,
    inner: &'a InnerNode,
}

impl<'a> Node<'a> {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn name(&self) -> Option<&'a str> {
        self.inner.name.as_ref().map(|name| name.as_str())
    }
    pub fn children(&self) -> NodeChildren {
        NodeChildren {
            document: self.document,
            inner: self.inner.children.iter(),
        }
    }
}

pub struct Scene<'a> {
    document: &'a InnerGltf,
    index: usize,
    inner: &'a InnerScene,
}

impl<'a> Scene<'a> {
    pub fn name(&self) -> Option<&'a str> {
        self.inner.name.as_ref().map(|name| name.as_str())
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn nodes(&self) -> NodeChildren {
        NodeChildren {
            document: self.document,
            inner: self.inner.nodes.iter(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("Deserialization failed: {0}")]
    DeserializeError(serde_json::Error),
}

pub fn import(path: &str) -> Result<Gltf, ImportError> {
    let file = File::open(path).map_err(ImportError::Io)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file into the generated structures
    let inner: InnerGltf =
        serde_json::from_reader(reader).map_err(ImportError::DeserializeError)?;

    Ok(Gltf { inner })
}
