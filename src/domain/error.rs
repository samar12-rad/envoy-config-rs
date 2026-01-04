use thiserror::Error;

use super::feature_flag::Environment;

/// These are the errors we can run into when working with feature flags.
/// The caller can pattern match on these to decide how to respond.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("feature flag key cannot be empty")]
    EmptyKey,

    #[error("feature flag key '{key}' already exists in environment '{environment}'")]
    DuplicateKey { key: String, environment: Environment },
}
