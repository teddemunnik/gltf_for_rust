use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SchemaUri {
    pub path: String,
    pub fragment: Option<String>,
}

impl SchemaUri {
    pub fn from_str(str: &str) -> SchemaUri {
        let pound = str.chars().position(|c| c == '#');
        match pound {
            Some(index) => SchemaUri { path: String::from(&str[..index]), fragment: Some(String::from(&str[(index + 1)..])) },
            None => SchemaUri { path: String::from(str), fragment: None }
        }
    }
}

impl std::fmt::Display for SchemaUri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)?;
        if let Some(fragment) = self.fragment.as_ref() {
            write!(f, "#{}", fragment)?;
        }

        Ok(())
    }
}

impl SchemaUri {
    pub fn is_schema_root(&self) -> bool {
        self.fragment.is_none()
    }
    pub fn definition_name(&self) -> Option<&str> {
        if let Some(fragment) = self.fragment.as_ref() {
            if let Some(definition) = fragment.strip_prefix("/definitions/") {
                return Some(definition);
            }
            if let Some(definition) = fragment.strip_prefix("/$defs/") {
                return Some(definition);
            }
        }
        None
    }
}
