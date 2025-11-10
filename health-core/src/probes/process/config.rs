use std::collections::HashMap;

/// Process check configuration
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    name: String,
}

impl ProcessConfig {
    /// Parse from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Self, String> {
        let name = params
            .get("name")
            .ok_or("missing required param: name")?
            .clone();

        if name.is_empty() {
            return Err("process name cannot be empty".to_string());
        }

        Ok(Self { name })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
