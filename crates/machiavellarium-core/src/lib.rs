//! Shared foundation for the Machiavellarium simulation workspace.

mod config;
mod error;

pub use config::{AppConfig, LoggingConfig, SimulationConfig};
pub use error::{Error, Result};
