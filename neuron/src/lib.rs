//! A biologically-inspired neural network simulator
//!
//! This library provides a realistic simulation of neurons with:
//! - Anatomical components (dendrites, soma, axon, synapses)
//! - Physiological properties (resting potential, action potentials, refractory period)
//! - Neurotransmitter systems (glutamate, GABA, dopamine, serotonin)
//!
//! # Examples
//!
//! ```
//! use neuron::{NeuralNetwork, Neurotransmitter};
//!
//! // Create a network with 3 neurons
//! let mut network = NeuralNetwork::new();
//! let n0 = network.add_neuron();
//! let n1 = network.add_neuron();
//! let n2 = network.add_neuron();
//!
//! // Connect them with synapses
//! network.connect(n0, n1, 0.8, Neurotransmitter::Glutamate);
//! network.connect(n1, n2, 0.5, Neurotransmitter::GABA);
//!
//! // Run simulation
//! network.step(&[(n0, 25.0)]);
//! ```

pub mod constants;
pub mod network;
pub mod neuron;
pub mod neurotransmitter;
pub mod synapse;

// Re-export main types for convenience
pub use network::NeuralNetwork;
pub use neuron::Neuron;
pub use neurotransmitter::Neurotransmitter;
pub use synapse::Synapse;
