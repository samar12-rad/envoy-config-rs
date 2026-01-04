use crate::domain::{Environment, FeatureFlag};

use super::FlagRepository;

/// A simple in-memory storage for feature flags - good enough for testing
/// and demos, but obviously everything gets lost when the server restarts.
#[derive(Default)]
pub struct InMemoryFlagRepository {
    flags: Vec<FeatureFlag>,
}

impl InMemoryFlagRepository {
    pub fn new() -> Self {
        Self { flags: Vec::new() }
    }
}

impl FlagRepository for InMemoryFlagRepository {
    fn upsert(&mut self, flag: FeatureFlag) -> FeatureFlag {
        if let Some(existing) = self
            .flags
            .iter_mut()
            .find(|f| f.environment == flag.environment && f.key == flag.key)
        {
            *existing = flag.clone();
            return flag;
        }

        self.flags.push(flag.clone());
        flag
    }

    fn list_by_environment(&self, env: Environment) -> Vec<FeatureFlag> {
        self.flags
            .iter()
            .filter(|f| f.environment == env)
            .cloned()
            .collect()
    }

    fn get(&self, env: &Environment, key: &str) -> Option<FeatureFlag> {
        self.flags
            .iter()
            .find(|f| &f.environment == env && f.key == key)
            .cloned()
    }
}
