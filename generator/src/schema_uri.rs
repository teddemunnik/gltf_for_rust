use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SchemaUri {
    pub base_path: String,
    pub relative_path: String,
    pub instance_path: Vec<String>,
}

impl std::fmt::Display for SchemaUri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.base_path, &self.relative_path)?;
        if !self.instance_path.is_empty() {
            write!(f, "#")?;
            for instance in &self.instance_path {
                write!(f, "/{}", instance)?;
            }
        }

        Ok(())
    }
}

impl SchemaUri {
    pub fn is_schema_root(&self) -> bool {
        self.instance_path.is_empty()
    }
    pub fn definition_name(&self) -> Option<&str> {
        if self.instance_path.len() >= 2 && self.instance_path[0] == "definitions"
            || self.instance_path[0] == "$defs"
        {
            Some(&self.instance_path[1])
        } else {
            None
        }
    }
}
