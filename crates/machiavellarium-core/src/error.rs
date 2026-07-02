use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read config at {path}")]
    ConfigRead {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse config")]
    ConfigParse(#[from] toml::de::Error),

    #[error("simulation error: {0}")]
    Simulation(String),
}

impl Error {
    pub fn simulation(message: impl Into<String>) -> Self {
        Self::Simulation(message.into())
    }
}
