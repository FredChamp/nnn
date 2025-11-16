//! A biologically-inspired neural network simulator
//!
//! This library provides a realistic simulation of neurons and photoreceptors with:
//! - Anatomical components (dendrites, soma, axon, synapses)
//! - Physiological properties (resting potential, action potentials, refractory period)
//! - Neurotransmitter systems (glutamate, GABA, dopamine, serotonin)
//! - Retinal photoreceptors (cone cells with phototransduction)
//!
//! # Examples
//!
//! ## Neural Network
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
//!
//! ## Cone Photoreceptors
//! ```
//! use neuron::{Cone, ConeType, LightStimulus};
//!
//! // Create an L-cone (red-sensitive)
//! let mut cone = Cone::new(0, ConeType::L);
//!
//! // Stimulate with red light
//! cone.phototransduction(LightStimulus::red(100.0));
//!
//! // Check response
//! println!("Membrane potential: {} mV", cone.membrane_potential());
//! ```

pub mod cone;
pub mod constants;
pub mod network;
pub mod neuron;
pub mod neurotransmitter;
pub mod photopigment;
pub mod synapse;

// Re-export main types for convenience
pub use cone::Cone;
pub use network::NeuralNetwork;
pub use neuron::Neuron;
pub use neurotransmitter::Neurotransmitter;
pub use photopigment::{ConeType, LightStimulus};
pub use synapse::Synapse;
