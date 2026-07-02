use machiavellarium_core::{AppConfig, Result};

/// Outcome of a single simulation step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepOutcome {
    pub step: u64,
    pub finished: bool,
}

/// Builder for constructing a [`Simulation`] from configuration.
#[derive(Debug, Default)]
pub struct SimulationBuilder {
    config: Option<AppConfig>,
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Simulation> {
        let config = self
            .config
            .ok_or_else(|| machiavellarium_core::Error::simulation("config is required"))?;
        Simulation::new(config)
    }
}

/// Minimal simulation scaffold: holds config and step counter until agents are wired in.
#[derive(Debug)]
pub struct Simulation {
    config: AppConfig,
    step: u64,
}

impl Simulation {
    pub fn new(config: AppConfig) -> Result<Self> {
        tracing::info!(
            seed = config.simulation.seed,
            agents = config.agents.count,
            width = config.environment.width,
            height = config.environment.height,
            "initializing simulation"
        );
        Ok(Self { config, step: 0 })
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn step(&self) -> u64 {
        self.step
    }

    pub fn step_once(&mut self) -> StepOutcome {
        self.step += 1;
        let finished = self.step >= self.config.simulation.max_steps;
        StepOutcome {
            step: self.step,
            finished,
        }
    }

    pub fn run(&mut self) -> Result<Vec<StepOutcome>> {
        let mut outcomes = Vec::with_capacity(self.config.simulation.max_steps as usize);
        loop {
            let outcome = self.step_once();
            let finished = outcome.finished;
            outcomes.push(outcome);
            if finished {
                break;
            }
        }
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use machiavellarium_core::AppConfig;

    fn test_config(max_steps: u64) -> AppConfig {
        format!(
            r#"
[simulation]
seed = 7
max_steps = {max_steps}
tick_rate_hz = 10.0

[environment]
width = 8
height = 8

[agents]
count = 2

[logging]
level = "info"
"#
        )
        .parse()
        .expect("test config should parse")
    }

    #[test]
    fn runs_for_configured_steps() {
        let mut sim = Simulation::new(test_config(3)).expect("simulation should initialize");
        let outcomes = sim.run().expect("run should succeed");
        assert_eq!(outcomes.len(), 3);
        assert!(outcomes.last().expect("outcomes").finished);
    }
}
