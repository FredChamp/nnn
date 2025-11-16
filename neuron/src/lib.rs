//! A biologically-inspired neural network simulator
//!
//! This library provides a realistic simulation of neurons and visual processing with:
//! - Anatomical components (dendrites, soma, axon, synapses)
//! - Physiological properties (resting potential, action potentials, refractory period)
//! - Neurotransmitter systems (glutamate, GABA, dopamine, serotonin)
//! - Retinal photoreceptors (cone cells with phototransduction)
//! - Complete visual pathway (retina → ganglion cells → V1 cortex)
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
//!
//! ## Visual Processing
//! ```no_run
//! use neuron::visual_pathway::{VisualPathway, test_patterns};
//!
//! // Create visual system
//! let mut pathway = VisualPathway::new(32, 32);
//!
//! // Process a vertical bar
//! let image = test_patterns::vertical_bar(32, 32);
//! let response = pathway.process_grayscale_image(&image);
//!
//! println!("Dominant orientation: {}", response.features.dominant_orientation());
//! ```

pub mod cone;
pub mod constants;
pub mod ganglion;
pub mod image_utils;
pub mod network;
pub mod neuron;
pub mod neurotransmitter;
pub mod photopigment;
pub mod synapse;
pub mod v1_cortex;
pub mod v2_cortex;
pub mod visual_pathway;

// Re-export main types for convenience
pub use cone::Cone;
pub use ganglion::{GanglionCell, GanglionLayer, GanglionType};
pub use network::NeuralNetwork;
pub use neuron::Neuron;
pub use neurotransmitter::Neurotransmitter;
pub use photopigment::{ConeType, LightStimulus};
pub use synapse::Synapse;
pub use v1_cortex::{Orientation, V1Cortex, V1Neuron, V1NeuronType};
pub use v2_cortex::{CornerType, V2Cortex, V2Response};
pub use visual_pathway::VisualPathway;
