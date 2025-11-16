use neuron::{Cone, ConeType, LightStimulus, NeuralNetwork, Neurotransmitter};

fn neuron_example() {
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

    println!("\n✅ Neural simulation completed\n");
}

fn cone_example() {
    println!("👁️  Retinal Cone Simulation\n");

    // Create three types of cones (RGB color vision)
    let mut s_cone = Cone::new(0, ConeType::S); // Blue
    let mut m_cone = Cone::new(1, ConeType::M); // Green
    let mut l_cone = Cone::new(2, ConeType::L); // Red

    println!("Created 3 cones:");
    println!("  - S-cone (Blue-sensitive, peak: {} nm)", ConeType::S.peak_wavelength());
    println!("  - M-cone (Green-sensitive, peak: {} nm)", ConeType::M.peak_wavelength());
    println!("  - L-cone (Red-sensitive, peak: {} nm)\n", ConeType::L.peak_wavelength());

    // Test 1: Darkness
    println!("=== Test 1: Darkness ===");
    for _ in 0..5 {
        s_cone.phototransduction(LightStimulus::darkness());
        m_cone.phototransduction(LightStimulus::darkness());
        l_cone.phototransduction(LightStimulus::darkness());
    }
    print_cone_status("S-cone", &s_cone);
    print_cone_status("M-cone", &m_cone);
    print_cone_status("L-cone", &l_cone);

    // Test 2: Blue light
    println!("\n=== Test 2: Blue Light (470nm, intensity: 80) ===");
    for _ in 0..20 {
        s_cone.phototransduction(LightStimulus::blue(80.0));
        m_cone.phototransduction(LightStimulus::blue(80.0));
        l_cone.phototransduction(LightStimulus::blue(80.0));
    }
    print_cone_status("S-cone", &s_cone);
    print_cone_status("M-cone", &m_cone);
    print_cone_status("L-cone", &l_cone);
    println!("👉 S-cone responds most to blue light!");

    // Test 3: Green light
    println!("\n=== Test 3: Green Light (530nm, intensity: 80) ===");
    // Reset cones to dark state
    s_cone = Cone::new(0, ConeType::S);
    m_cone = Cone::new(1, ConeType::M);
    l_cone = Cone::new(2, ConeType::L);
    
    for _ in 0..20 {
        s_cone.phototransduction(LightStimulus::green(80.0));
        m_cone.phototransduction(LightStimulus::green(80.0));
        l_cone.phototransduction(LightStimulus::green(80.0));
    }
    print_cone_status("S-cone", &s_cone);
    print_cone_status("M-cone", &m_cone);
    print_cone_status("L-cone", &l_cone);
    println!("👉 M-cone responds most to green light!");

    // Test 4: Red light
    println!("\n=== Test 4: Red Light (650nm, intensity: 80) ===");
    s_cone = Cone::new(0, ConeType::S);
    m_cone = Cone::new(1, ConeType::M);
    l_cone = Cone::new(2, ConeType::L);
    
    for _ in 0..20 {
        s_cone.phototransduction(LightStimulus::red(80.0));
        m_cone.phototransduction(LightStimulus::red(80.0));
        l_cone.phototransduction(LightStimulus::red(80.0));
    }
    print_cone_status("S-cone", &s_cone);
    print_cone_status("M-cone", &m_cone);
    print_cone_status("L-cone", &l_cone);
    println!("👉 L-cone responds most to red light!");

    // Test 5: White light and adaptation
    println!("\n=== Test 5: White Light & Adaptation (100 time steps) ===");
    s_cone = Cone::new(0, ConeType::S);
    m_cone = Cone::new(1, ConeType::M);
    l_cone = Cone::new(2, ConeType::L);
    
    for step in 0..100 {
        s_cone.phototransduction(LightStimulus::white_light(100.0));
        m_cone.phototransduction(LightStimulus::white_light(100.0));
        l_cone.phototransduction(LightStimulus::white_light(100.0));
        
        if step == 0 || step == 50 || step == 99 {
            println!("\nStep {}: ", step);
            print_cone_status("S-cone", &s_cone);
            print_cone_status("M-cone", &m_cone);
            print_cone_status("L-cone", &l_cone);
        }
    }
    println!("👉 Cones adapt to sustained light!");

    println!("\n✅ Cone simulation completed\n");
}

fn cone_to_neuron_example() {
    println!("🔗 Cone-to-Neuron Integration Example\n");

    // Create a simple retina-like circuit:
    // 3 cones (S, M, L) → bipolar neurons → ganglion cell

    let mut s_cone = Cone::new(0, ConeType::S);
    let mut m_cone = Cone::new(1, ConeType::M);
    let mut l_cone = Cone::new(2, ConeType::L);

    let mut network = NeuralNetwork::new();
    
    // Create bipolar-like neurons (one for each cone)
    let bipolar_s = network.add_neuron();
    let bipolar_m = network.add_neuron();
    let bipolar_l = network.add_neuron();
    
    // Create a ganglion-like cell that integrates all inputs
    let ganglion = network.add_neuron();

    // Connect bipolars to ganglion
    network.connect(bipolar_s, ganglion, 0.8, Neurotransmitter::Glutamate);
    network.connect(bipolar_m, ganglion, 0.8, Neurotransmitter::Glutamate);
    network.connect(bipolar_l, ganglion, 0.8, Neurotransmitter::Glutamate);

    // Connect cones to bipolar neurons
    s_cone.connect_to_neuron(bipolar_s);
    m_cone.connect_to_neuron(bipolar_m);
    l_cone.connect_to_neuron(bipolar_l);

    println!("Circuit created:");
    println!("  S-cone → Bipolar-S \\");
    println!("  M-cone → Bipolar-M  → Ganglion cell");
    println!("  L-cone → Bipolar-L /\n");

    // Simulate different light conditions
    let scenarios = vec![
        ("Darkness", LightStimulus::darkness()),
        ("Blue light", LightStimulus::blue(100.0)),
        ("Green light", LightStimulus::green(100.0)),
        ("Red light", LightStimulus::red(100.0)),
        ("White light", LightStimulus::white_light(100.0)),
    ];

    for (name, light) in scenarios {
        println!("=== {} ===", name);
        
        // Reset
        s_cone = Cone::new(0, ConeType::S);
        m_cone = Cone::new(1, ConeType::M);
        l_cone = Cone::new(2, ConeType::L);
        s_cone.connect_to_neuron(bipolar_s);
        m_cone.connect_to_neuron(bipolar_m);
        l_cone.connect_to_neuron(bipolar_l);

        // Stimulate cones and propagate to network
        for _ in 0..10 {
            s_cone.phototransduction(light);
            m_cone.phototransduction(light);
            l_cone.phototransduction(light);

            // Collect cone outputs
            let mut inputs = Vec::new();
            inputs.extend(s_cone.transmit_to_neurons());
            inputs.extend(m_cone.transmit_to_neurons());
            inputs.extend(l_cone.transmit_to_neurons());

            network.step(&inputs);
        }

        println!("Cone responses:");
        println!("  S-cone: {:.1}% active", s_cone.response_level() * 100.0);
        println!("  M-cone: {:.1}% active", m_cone.response_level() * 100.0);
        println!("  L-cone: {:.1}% active", l_cone.response_level() * 100.0);
        println!("Ganglion cell: V={:.1}mV, Rate={:.1}Hz\n",
                 network.get_neuron(ganglion).membrane_potential(),
                 network.get_neuron(ganglion).firing_rate(100));
    }

    println!("✅ Integration simulation completed\n");
}

fn print_cone_status(name: &str, cone: &Cone) {
    println!("  {}: V={:.1}mV, cGMP={:.1}, Glutamate={:.1}, Response={:.0}%, Adapted={}",
             name,
             cone.membrane_potential(),
             cone.cgmp_level(),
             cone.glutamate_release(),
             cone.response_level() * 100.0,
             cone.is_light_adapted());
}

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   Neuron & Retinal Cone Simulation Demo          ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // Run all examples
    neuron_example();
    println!("\n{}\n", "=".repeat(60));
    
    cone_example();
    println!("\n{}\n", "=".repeat(60));
    
    cone_to_neuron_example();

    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   All simulations completed successfully! 🎉     ║");
    println!("╚═══════════════════════════════════════════════════╝");
}
