use crate::probes::ProbeResult;
use std::collections::HashMap;

/// Trait that all health checks must implement
pub trait HealthCheck: Send + Sync {
    /// Execute the health check
    fn check(&self) -> ProbeResult;
    
    /// Get the name/type of this check
    fn name(&self) -> &str;
}

/// Type alias for check factory functions
pub type CheckFactory = Box<dyn Fn(&HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> + Send + Sync>;

/// Registry for all available health check types
pub struct CheckRegistry {
    factories: HashMap<String, CheckFactory>,
}

impl CheckRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// Register a new check type with its factory function
    pub fn register<F>(&mut self, check_type: &str, factory: F)
    where
        F: Fn(&HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> + Send + Sync + 'static,
    {
        self.factories.insert(check_type.to_string(), Box::new(factory));
    }

    /// Create a check instance from config parameters
    pub fn create_check(&self, check_type: &str, params: &HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> {
        self.factories
            .get(check_type)
            .ok_or_else(|| format!("unknown check type: {check_type}"))
            .and_then(|factory| factory(params))
    }

    /// Get all registered check types
    pub fn available_checks(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

impl Default for CheckRegistry {
    fn default() -> Self {
        Self::new()
    }
}
