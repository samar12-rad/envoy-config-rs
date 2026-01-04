use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Environments a flag can target. Serialized as lowercase strings for easy APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Environment::Dev => "dev",
            Environment::Staging => "staging",
            Environment::Prod => "prod",
        };
        write!(f, "{}", text)
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "staging" => Ok(Environment::Staging),
            "prod" => Ok(Environment::Prod),
            _ => Err(format!("unsupported environment: {s}")),
        }
    }
}

/// A single feature flag scoped to an environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub key: String,
    pub enabled: bool,
    pub environment: Environment,
}

impl FeatureFlag {
    pub fn new(key: String, enabled: bool, environment: Environment) -> Self {
        Self {
            key,
            enabled,
            environment,
        }
    }
}
