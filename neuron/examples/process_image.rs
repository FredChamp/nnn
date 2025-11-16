//! Example: Process a real image through the visual system

use neuron::image_utils::{ascii_visualization, load_and_resize_grayscale, save_grayscale_image};
use neuron::visual_pathway::VisualPathway;
use std::env;

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   Real Image Visual Processing Demo              ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // Get image path from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <image_path>", args[0]);
        println!("\nExample:");
        println!("  cargo run --example process_image --release -- images/input/myimage.jpg");
        println!("\nThe image will be:");
        println!("  1. Loaded and resized to 64x64 pixels");
        println!("  2. Processed through the complete visual pathway");
        println!("  3. Edge map saved to 'images/output/edges.png'");
        println!("  4. Results displayed\n");
        return;
    }

    let image_path = &args[1];
    println!("📷 Loading image: {}\n", image_path);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("images/output").expect("Failed to create images/output directory");

    // Processing parameters
    let size = 64; // Process at 64x64 for reasonable speed
    
    // Load and resize image
    let image = match load_and_resize_grayscale(image_path, size, size) {
        Ok(img) => {
            println!("✓ Image loaded and resized to {}x{}", size, size);
            img
        }
        Err(e) => {
            eprintln!("❌ Error loading image: {}", e);
            return;
        }
    };

    // Show ASCII preview
    println!("\n📊 Input Image Preview (ASCII):");
    println!("{}", ascii_visualization(&image, 60));

    // Create visual pathway
    println!("🧠 Initializing visual processing system...");
    let mut pathway = VisualPathway::new(size as usize, size as usize);
    println!("✓ Visual pathway created");
    println!("  - {} cones (photoreceptors)", size * size);
    println!("  - Ganglion layer (edge detection)");
    println!("  - V1 cortex (orientation detection)\n");

    // Process image
    println!("⚡ Processing image through visual pathway...\n");
    let response = pathway.process_grayscale_image(&image);

    // Display results
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   VISUAL PROCESSING RESULTS                       ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    println!("1️⃣  Photoreceptor Layer (Cones)");
    let total_activation: f32 = response.cone_activations.iter().flatten().sum();
    let avg_activation = total_activation / (size * size) as f32;
    println!("   Average cone activation: {:.1}%", avg_activation * 100.0);
    
    let active_cones = response.cone_activations
        .iter()
        .flatten()
        .filter(|&&x| x > 0.1)
        .count();
    println!("   Active cones: {}/{}", active_cones, size * size);

    println!("\n2️⃣  Ganglion Cell Layer (Edge Detection)");
    let total_edges: f32 = response.edge_map.iter().flatten().sum();
    let avg_edge = total_edges / (size * size) as f32;
    println!("   Average edge strength: {:.3}", avg_edge);
    
    let edge_pixels = response.edge_map
        .iter()
        .flatten()
        .filter(|&&x| x > 0.1)
        .count();
    println!("   Edge pixels detected: {}/{}", edge_pixels, size * size);

    // Save edge map
    let output_path = "images/output/edges.png";
    if let Err(e) = save_grayscale_image(&response.edge_map, output_path) {
        eprintln!("   Warning: Could not save edge map: {}", e);
    } else {
        println!("   ✓ Edge map saved to '{}'", output_path);
    }

    println!("\n3️⃣  V1 Primary Visual Cortex (Orientation Detection)");
    let active_v1 = response.orientation_map
        .iter()
        .flatten()
        .filter(|o| o.is_some())
        .count();
    println!("   Active V1 regions: {}/{}", active_v1, size * size);
    
    println!("\n   Orientation Analysis:");
    println!("   ├─ Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   ├─ Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   └─ Diagonal edges:   {:.2}", response.features.diagonal_strength);

    println!("\n4️⃣  High-Level Features");
    println!("   Total edge strength: {:.2}", response.features.edge_strength());
    println!("   Dominant orientation: {}", response.features.dominant_orientation());
    println!("   Overall activation: {:.2}", response.features.total_activation);

    // Interpret the scene
    println!("\n🔍 Scene Interpretation:");
    interpret_scene(&response.features);

    // Show edge map preview
    println!("\n📊 Edge Map Preview (ASCII):");
    println!("{}", ascii_visualization(&response.edge_map, 60));

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║   Processing Complete! ✨                         ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
}

fn interpret_scene(features: &neuron::visual_pathway::VisualFeatures) {
    let edge_strength = features.edge_strength();
    let h = features.horizontal_strength;
    let v = features.vertical_strength;
    let d = features.diagonal_strength;

    if edge_strength < 0.5 {
        println!("   • Low edge content - possibly a uniform or blurry image");
    } else if edge_strength > 10.0 {
        println!("   • High edge content - detailed or textured image");
    }

    if h > v * 1.5 && h > d * 1.5 {
        println!("   • Strong horizontal structures detected");
        println!("   • Possible content: landscape, horizon, floors, ceilings");
    } else if v > h * 1.5 && v > d * 1.5 {
        println!("   • Strong vertical structures detected");
        println!("   • Possible content: buildings, trees, pillars, walls");
    } else if d > h * 1.5 && d > v * 1.5 {
        println!("   • Strong diagonal structures detected");
        println!("   • Possible content: stairs, roofs, tilted objects");
    } else if (h - v).abs() < h * 0.3 && h > d {
        println!("   • Balanced horizontal and vertical content");
        println!("   • Possible content: grid patterns, windows, architecture");
    } else {
        println!("   • Mixed orientations - complex scene");
    }

    // Activity level
    if features.total_activation > 20.0 {
        println!("   • High cortical activation - salient visual features present");
    } else if features.total_activation < 5.0 {
        println!("   • Low cortical activation - simple or minimal features");
    }
}
