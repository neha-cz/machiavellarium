use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::error::{Error, Result};

/// Top-level application configuration loaded from TOML.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AppConfig {
    pub simulation: SimulationConfig,
    pub environment: EnvironmentConfig,
    pub agents: AgentsConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents =
            std::fs::read_to_string(path.as_ref()).map_err(|source| Error::ConfigRead {
                path: path.as_ref().display().to_string(),
                source,
            })?;
        contents.parse()
    }
}

impl FromStr for AppConfig {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        toml::from_str(contents).map_err(Error::ConfigParse)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SimulationConfig {
    pub seed: u64,
    pub max_steps: u64,
    pub tick_rate_hz: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AgentsConfig {
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[simulation]
seed = 1
max_steps = 100
tick_rate_hz = 10.0

[environment]
width = 16
height = 16

[agents]
count = 2

[logging]
level = "debug"
"#;

    #[test]
    fn parses_valid_config() {
        let config: AppConfig = SAMPLE.parse().expect("sample config should parse");
        assert_eq!(config.simulation.seed, 1);
        assert_eq!(config.agents.count, 2);
        assert_eq!(config.logging.level, "debug");
    }
}
