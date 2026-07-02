use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use machiavellarium_core::AppConfig;
use machiavellarium_env::SimulationBuilder;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(
    name = "machiavellarium",
    about = "Multi-agent social simulation for emergent learning experiments"
)]
struct Cli {
    /// Path to the TOML configuration file.
    #[arg(
        short,
        long,
        env = "MACHIAVELLARIUM_CONFIG",
        default_value = "configs/default.toml"
    )]
    config: PathBuf,

    /// Run the simulation to completion.
    #[arg(long)]
    run: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::from_file(&cli.config)
        .with_context(|| format!("loading config from {}", cli.config.display()))?;

    init_tracing(&config.logging.level)?;

    tracing::info!(config = %cli.config.display(), "starting machiavellarium");

    let mut simulation = SimulationBuilder::new()
        .config(config)
        .build()
        .context("building simulation")?;

    if cli.run {
        let outcomes = simulation.run().context("running simulation")?;
        tracing::info!(steps = outcomes.len(), "simulation finished");
    } else {
        tracing::info!("simulation initialized (pass --run to execute)");
    }

    Ok(())
}

fn init_tracing(level: &str) -> anyhow::Result<()> {
    let filter = EnvFilter::try_new(format!("machiavellarium={level}"))
        .or_else(|_| EnvFilter::try_new("info"))
        .context("building tracing filter")?;

    if tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init()
        .is_err()
    {
        // Another subscriber may already be registered (e.g. in tests).
    }

    Ok(())
}
