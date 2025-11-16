use neuron::{NeuralNetwork, Neurotransmitter};

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
