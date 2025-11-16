//! Neural network implementation for simulating interconnected neurons

use crate::neuron::Neuron;
use crate::neurotransmitter::Neurotransmitter;

/// A neural network consisting of interconnected neurons
pub struct NeuralNetwork {
    neurons: Vec<Neuron>,
    time_ms: u32,
}

impl NeuralNetwork {
    /// Creates a new empty neural network
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            time_ms: 0,
        }
    }

    /// Returns the current simulation time in milliseconds
    pub fn current_time(&self) -> u32 {
        self.time_ms
    }

    /// Returns the number of neurons in the network
    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }

    /// Adds a new neuron to the network
    ///
    /// # Returns
    /// The ID of the newly created neuron
    pub fn add_neuron(&mut self) -> usize {
        let id = self.neurons.len();
        self.neurons.push(Neuron::new(id));
        id
    }

    /// Creates a synaptic connection between two neurons
    ///
    /// # Arguments
    /// * `from` - ID of the presynaptic neuron
    /// * `to` - ID of the postsynaptic neuron
    /// * `weight` - Synaptic weight
    /// * `neurotransmitter` - Type of neurotransmitter
    ///
    /// # Panics
    /// Panics if either neuron ID is out of bounds
    pub fn connect(
        &mut self,
        from: usize,
        to: usize,
        weight: f32,
        neurotransmitter: Neurotransmitter,
    ) {
        assert!(from < self.neurons.len(), "Source neuron ID out of bounds");
        assert!(to < self.neurons.len(), "Target neuron ID out of bounds");
        
        self.neurons[from].connect_to(to, weight, neurotransmitter);
    }

    /// Returns a reference to a specific neuron
    ///
    /// # Panics
    /// Panics if the neuron ID is out of bounds
    pub fn get_neuron(&self, id: usize) -> &Neuron {
        &self.neurons[id]
    }

    /// Simulates one time step of the network
    ///
    /// # Arguments
    /// * `external_inputs` - Slice of (neuron_id, signal) pairs representing external stimulation
    pub fn step(&mut self, external_inputs: &[(usize, f32)]) {
        // Phase 1: Apply external inputs to dendrites
        for &(neuron_id, signal) in external_inputs {
            if neuron_id < self.neurons.len() {
                self.neurons[neuron_id].receive_input(signal);
            }
        }

        // Phase 2: Integrate inputs and generate action potentials
        let mut transmissions = Vec::new();
        for neuron in &mut self.neurons {
            neuron.integrate_inputs();
            if neuron.generate_action_potential(self.time_ms) {
                transmissions.extend(neuron.transmit());
            }
        }

        // Phase 3: Deliver synaptic transmissions
        for (target_id, signal, _neurotransmitter) in transmissions {
            if target_id < self.neurons.len() {
                self.neurons[target_id].receive_input(signal);
            }
        }

        self.time_ms += 1;
    }

    /// Runs the simulation for a specified duration
    ///
    /// # Arguments
    /// * `duration_ms` - Duration to simulate in milliseconds
    /// * `input_fn` - Function that returns external inputs for each time step
    pub fn run<F>(&mut self, duration_ms: u32, mut input_fn: F)
    where
        F: FnMut(u32) -> Vec<(usize, f32)>,
    {
        for _ in 0..duration_ms {
            let inputs = input_fn(self.time_ms);
            self.step(&inputs);
        }
    }

    /// Prints the current state of all neurons
    pub fn print_status(&self) {
        println!("\n=== Time: {} ms ===", self.time_ms);
        for neuron in &self.neurons {
            println!(
                "Neuron {}: V={:.1}mV, Refractory={}, Rate={:.1}Hz",
                neuron.id(),
                neuron.membrane_potential(),
                neuron.is_refractory(),
                neuron.firing_rate(100)
            );
        }
    }

    /// Returns an iterator over all neurons
    pub fn neurons(&self) -> impl Iterator<Item = &Neuron> {
        self.neurons.iter()
    }
}

impl Default for NeuralNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let network = NeuralNetwork::new();
        assert_eq!(network.neuron_count(), 0);
        assert_eq!(network.current_time(), 0);
    }

    #[test]
    fn test_add_neurons() {
        let mut network = NeuralNetwork::new();
        let n0 = network.add_neuron();
        let n1 = network.add_neuron();
        
        assert_eq!(n0, 0);
        assert_eq!(n1, 1);
        assert_eq!(network.neuron_count(), 2);
    }

    #[test]
    fn test_connect_neurons() {
        let mut network = NeuralNetwork::new();
        let n0 = network.add_neuron();
        let n1 = network.add_neuron();
        
        network.connect(n0, n1, 0.8, Neurotransmitter::Glutamate);
        assert_eq!(network.get_neuron(n0).synapse_count(), 1);
    }

    #[test]
    fn test_simulation_step() {
        let mut network = NeuralNetwork::new();
        network.add_neuron();
        
        let initial_time = network.current_time();
        network.step(&[]);
        assert_eq!(network.current_time(), initial_time + 1);
    }

    #[test]
    fn test_run_simulation() {
        let mut network = NeuralNetwork::new();
        let n0 = network.add_neuron();
        
        network.run(10, |t| {
            if t % 5 == 0 {
                vec![(n0, 25.0)]
            } else {
                vec![]
            }
        });
        
        assert_eq!(network.current_time(), 10);
    }
}
