use crate::domain::{Environment, FeatureFlag};

pub mod in_memory;

/// Abstraction over flag persistence so the service is decoupled from storage details.
pub trait FlagRepository {
    fn upsert(&mut self, flag: FeatureFlag) -> FeatureFlag;
    fn list_by_environment(&self, env: Environment) -> Vec<FeatureFlag>;
    fn get(&self, env: &Environment, key: &str) -> Option<FeatureFlag>;
}

pub use in_memory::InMemoryFlagRepository;
