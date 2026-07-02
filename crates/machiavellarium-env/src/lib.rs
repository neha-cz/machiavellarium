//! Simulated environment crate.
//!
//! Agent logic and learning will live in separate crates; this crate owns world
//! state, the simulation loop, and interaction scheduling.

mod simulation;

pub use simulation::{Simulation, SimulationBuilder, StepOutcome};
