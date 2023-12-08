
use schemars::{schema::{SchemaObject, RootSchema}, visit::{Visitor, visit_schema_object}};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use schemars::visit::visit_root_schema;

#[derive(Debug)]
enum Error {
    FailedToResolveReference(PathBuf),
    SomeError
}

struct ResolveSchemaReferencesVisitor {
    parent_path: PathBuf,
    pending_result: Result<(), Error>
}

impl ResolveSchemaReferencesVisitor{
    fn for_schema_path<P : AsRef<Path>>(path: P) -> Self where PathBuf: From<P>{
        // Remove the file part of the schema path. This is the path that relative schema references
        // for this schema will be relative to
        let mut current_directory = PathBuf::from(path);
        current_directory.pop();

        Self{
            parent_path: current_directory,
            pending_result : Ok(())
        }
    }
}



impl Visitor for ResolveSchemaReferencesVisitor{
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        // If an earlier sub schema failed to load early out, as we'll be aborting anyway
        if self.pending_result.is_err(){
            return;
        }

        // For non reference schemas we need to recursively visit children to detect possible schema
        // references there.
        if !schema.is_ref(){
            visit_schema_object(self, schema);
            return;
        }

        // Figure out the full path for the schema reference. If it is a relative path, we expect it
        // to be relative to the schema that's referencing it.
        let reference = schema.reference.as_ref().unwrap().as_str();
        let resolved_path= self.parent_path.join(Path::new(reference));

        let child_schema_result = read_and_resolve_refs(&resolved_path);
        if child_schema_result.is_err(){
            self.pending_result = Err(Error::FailedToResolveReference(resolved_path));
            return;
        }

        // Replace the reference schema with the loaded schema
        *schema = child_schema_result.unwrap().schema;
    }
}
fn read_and_resolve_refs<P: AsRef<Path> + Copy>(path: P) -> Result<RootSchema, Error> where PathBuf: From<P>{
    let file = File::open(path).map_err(|e| Error::SomeError)?;
    let reader = BufReader::new(file);
    let mut root_schema = serde_json::from_reader(reader).map_err(|e| Error::SomeError)?;

    let mut visitor = ResolveSchemaReferencesVisitor::for_schema_path(path);
    visit_root_schema(&mut visitor, &mut root_schema);

    visitor.pending_result.map(|a| root_schema)
}

fn main(){
    // Load the root schema
    let schema = read_and_resolve_refs("vendor/gltf/specification/2.0/schema/gLTF.schema.json");
    print!("{:#?}", schema);
}