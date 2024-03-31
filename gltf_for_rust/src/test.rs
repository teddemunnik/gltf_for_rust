use crate::generated::gltf::Gltf;
use crate::GltfObject;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
fn test() {
    // Open the file in read-only mode with buffer.
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(
        "../vendor/gltf_sample_assets/Models/XmpMetadataRoundedCube/glTF/XmpMetadataRoundedCube.gltf",
    );

    println!("{}", d.display());

    let file = File::open(d).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Gltf = serde_json::from_reader(reader).unwrap();

    let extension: crate::generated::khr_xmp_json_ld::gltf::Extension =
        u.parse_extension().unwrap();

    println!("{:#?}", extension);
}
