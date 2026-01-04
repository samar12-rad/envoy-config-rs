use std::sync::RwLock;

use crate::domain::{DomainError, Environment, FeatureFlag};
use crate::repository::FlagRepository;

/// Our service layer handles all the business logic and keeps the database layer hidden away.
/// We use RwLock here so we can share the repository across multiple threads without
/// needing to wrap everything in Arc<Mutex<_>> all over the place.
pub struct FlagService<R: FlagRepository> {
    repository: RwLock<R>,
}

impl<R: FlagRepository> FlagService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository: RwLock::new(repository),
        }
    }

    /// Create or update a flag. We do some basic validation first
    /// to make sure the key isn't empty, then check if it already exists.
    pub fn set_flag(
        &self,
        key: String,
        enabled: bool,
        environment: Environment,
    ) -> Result<FeatureFlag, DomainError> {
        if key.trim().is_empty() {
            return Err(DomainError::EmptyKey);
        }

        let mut repo = self
            .repository
            .write()
            .expect("flag repository lock unexpectedly poisoned");

        // Check for uniqueness here in the service layer - if the key already
        // exists in this environment, we just update it. Otherwise create a new one.
        if let Some(mut existing) = repo.get(&environment, &key) {
            existing.enabled = enabled;
            return Ok(repo.upsert(existing));
        }

        let flag = FeatureFlag::new(key, enabled, environment);
        Ok(repo.upsert(flag))
    }

    pub fn get_flags(&self, env: Environment) -> Vec<FeatureFlag> {
        let repo = self
            .repository
            .read()
            .expect("flag repository lock unexpectedly poisoned");
        repo.list_by_environment(env)
    }
}
