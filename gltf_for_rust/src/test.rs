use std::path::PathBuf;

use serde_json;

use crate::import::{import, Node};

fn visit(depth: usize, node: &Node) {
    println!(
        "{}- {}",
        " ".repeat(depth),
        node.name().unwrap_or("<unnamed>")
    );

    for child in node.children() {
        visit(depth + 1, &child)
    }
}

#[test]
fn test() {
    // Open the file in read-only mode with buffer.
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../vendor/gltf_sample_assets/Models/ABeautifulGame/glTF/ABeautifulGame.gltf");

    let gltf = import(d.to_str().unwrap()).unwrap();

    for scene in gltf.scenes() {
        println!("- {}", scene.name().unwrap_or("<unnamed scene>"));
        for node in scene.nodes() {
            visit(1, &node);
        }
    }
}
