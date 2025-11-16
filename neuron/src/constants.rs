//! Physiological constants for neuron simulation

/// Resting membrane potential in millivolts
pub const RESTING_POTENTIAL: f32 = -70.0;

/// Threshold potential for action potential generation in millivolts
pub const THRESHOLD: f32 = -55.0;

/// Peak voltage during action potential in millivolts
pub const ACTION_POTENTIAL_PEAK: f32 = 30.0;

/// Hyperpolarization potential in millivolts
pub const HYPERPOLARIZATION: f32 = -75.0;

/// Duration of refractory period in milliseconds
pub const REFRACTORY_PERIOD_MS: u32 = 2;

/// Maximum number of spikes to keep in history
pub const MAX_SPIKE_HISTORY: usize = 100;
