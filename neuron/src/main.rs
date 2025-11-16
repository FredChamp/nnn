use std::collections::VecDeque;

// Physiological constants
const RESTING_POTENTIAL: f32 = -70.0;
const THRESHOLD: f32 = -55.0;
const ACTION_POTENTIAL_PEAK: f32 = 30.0;
const HYPERPOLARIZATION: f32 = -75.0;
const REFRACTORY_PERIOD_MS: u32 = 2;

#[derive(Debug, Clone)]
enum Neurotransmitter {
    Glutamate,  // Excitatory
    GABA,       // Inhibitory
    Dopamine,
    Serotonin,
}

#[derive(Debug)]
struct Synapse {
    weight: f32,  // Connection strength
    neurotransmitter: Neurotransmitter,
    target: usize,  // Target neuron ID
}

#[derive(Debug)]
struct Neuron {
    id: usize,
    // Anatomy
    dendrites: Vec<f32>,  // Received signals
    soma_potential: f32,   // Membrane potential
    axon_signal: Option<f32>,  // Signal to transmit
    synapses: Vec<Synapse>,  // Output connections
    
    // Physiology
    is_refractory: bool,
    refractory_timer: u32,
    spike_history: VecDeque<u32>,  // Action potential history
}

impl Neuron {
    fn new(id: usize) -> Self {
        Self {
            id,
            dendrites: Vec::new(),
            soma_potential: RESTING_POTENTIAL,
            axon_signal: None,
            synapses: Vec::new(),
            is_refractory: false,
            refractory_timer: 0,
            spike_history: VecDeque::with_capacity(100),
        }
    }

    // Connects this neuron to another via a synapse
    fn connect_to(&mut self, target_id: usize, weight: f32, neurotransmitter: Neurotransmitter) {
        self.synapses.push(Synapse {
            weight,
            neurotransmitter,
            target: target_id,
        });
    }

    // Receives a signal on the dendrites
    fn receive_input(&mut self, signal: f32) {
        self.dendrites.push(signal);
    }

    // Integrates dendritic signals in the soma (spatial summation)
    fn integrate_inputs(&mut self) {
        if !self.dendrites.is_empty() {
            let sum: f32 = self.dendrites.iter().sum();
            self.soma_potential += sum / self.dendrites.len() as f32;
            self.dendrites.clear();
        }
    }

    // Generates an action potential according to the all-or-none law
    fn generate_action_potential(&mut self, time_ms: u32) -> bool {
        // Handle refractory period
        if self.is_refractory {
            self.refractory_timer -= 1;
            if self.refractory_timer == 0 {
                self.is_refractory = false;
                self.soma_potential = RESTING_POTENTIAL;
            }
            return false;
        }

        // All-or-none law
        if self.soma_potential >= THRESHOLD {
            // Depolarization phase
            self.soma_potential = ACTION_POTENTIAL_PEAK;
            self.axon_signal = Some(ACTION_POTENTIAL_PEAK);
            
            // Enter refractory period
            self.is_refractory = true;
            self.refractory_timer = REFRACTORY_PERIOD_MS;
            
            // Record the spike
            self.spike_history.push_back(time_ms);
            if self.spike_history.len() > 100 {
                self.spike_history.pop_front();
            }
            
            true
        } else {
            // Passive decay towards resting potential
            self.soma_potential += (RESTING_POTENTIAL - self.soma_potential) * 0.1;
            self.axon_signal = None;
            false
        }
    }

    // Transmits the signal via synapses
    fn transmit(&self) -> Vec<(usize, f32, Neurotransmitter)> {
        if let Some(signal) = self.axon_signal {
            self.synapses
                .iter()
                .map(|synapse| {
                    let modulated_signal = match synapse.neurotransmitter {
                        Neurotransmitter::Glutamate => signal * synapse.weight,
                        Neurotransmitter::GABA => -signal * synapse.weight * 0.5,
                        Neurotransmitter::Dopamine => signal * synapse.weight * 0.8,
                        Neurotransmitter::Serotonin => signal * synapse.weight * 0.6,
                    };
                    (synapse.target, modulated_signal, synapse.neurotransmitter.clone())
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    // Calculates the firing rate (in Hz)
    fn firing_rate(&self, window_ms: u32) -> f32 {
        if self.spike_history.is_empty() {
            return 0.0;
        }
        
        let last_time = *self.spike_history.back().unwrap();
        let threshold_time = last_time.saturating_sub(window_ms);
        
        let recent_spikes = self.spike_history
            .iter()
            .filter(|&&t| t >= threshold_time)
            .count();
        
        (recent_spikes as f32 / window_ms as f32) * 1000.0
    }
}

// Simple neural network
struct NeuralNetwork {
    neurons: Vec<Neuron>,
    time_ms: u32,
}

impl NeuralNetwork {
    fn new() -> Self {
        Self {
            neurons: Vec::new(),
            time_ms: 0,
        }
    }

    fn add_neuron(&mut self) -> usize {
        let id = self.neurons.len();
        self.neurons.push(Neuron::new(id));
        id
    }

    fn connect(&mut self, from: usize, to: usize, weight: f32, nt: Neurotransmitter) {
        self.neurons[from].connect_to(to, weight, nt);
    }

    // Simulates one time step
    fn step(&mut self, external_inputs: &[(usize, f32)]) {
        // 1. Apply external inputs
        for &(neuron_id, signal) in external_inputs {
            if neuron_id < self.neurons.len() {
                self.neurons[neuron_id].receive_input(signal);
            }
        }

        // 2. Integration and action potential generation
        let mut transmissions = Vec::new();
        for neuron in &mut self.neurons {
            neuron.integrate_inputs();
            if neuron.generate_action_potential(self.time_ms) {
                transmissions.extend(neuron.transmit());
            }
        }

        // 3. Synaptic transmission
        for (target_id, signal, _nt) in transmissions {
            if target_id < self.neurons.len() {
                self.neurons[target_id].receive_input(signal);
            }
        }

        self.time_ms += 1;
    }

    fn print_status(&self) {
        println!("\n=== Time: {} ms ===", self.time_ms);
        for neuron in &self.neurons {
            println!(
                "Neuron {}: V={:.1}mV, Refractory={}, Rate={:.1}Hz",
                neuron.id,
                neuron.soma_potential,
                neuron.is_refractory,
                neuron.firing_rate(100)
            );
        }
    }
}

fn main() {
    println!("🧠 Neural Network Simulation\n");

    let mut network = NeuralNetwork::new();

    // Create 3 neurons
    let n0 = network.add_neuron();
    let n1 = network.add_neuron();
    let n2 = network.add_neuron();

    // Synaptic connections
    network.connect(n0, n1, 0.8, Neurotransmitter::Glutamate);  // Excitatory
    network.connect(n0, n2, 0.6, Neurotransmitter::Glutamate);
    network.connect(n1, n2, 0.5, Neurotransmitter::GABA);       // Inhibitory

    println!("Network created: 3 neurons with synaptic connections");
    println!("N0 → N1 (Glutamate, excitatory)");
    println!("N0 → N2 (Glutamate, excitatory)");
    println!("N1 → N2 (GABA, inhibitory)\n");

    // Simulation over 50ms
    for t in 0..50 {
        let external_inputs = if t % 10 == 0 {
            // Periodic stimulation of neuron 0
            vec![(n0, 25.0)]
        } else {
            vec![]
        };

        network.step(&external_inputs);

        // Display every 10ms
        if t % 10 == 0 {
            network.print_status();
        }
    }

    println!("\n✅ Simulation completed");
}
