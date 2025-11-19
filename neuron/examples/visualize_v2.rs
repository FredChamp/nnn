//! Example: Visualize V2 cortex output (corners and contours)

use neuron::image_utils::{
    load_and_resize_grayscale, 
    visualize_corner_map, 
    visualize_contours,
    visualize_v2_composite
};
use neuron::visual_pathway::VisualPathway;
use std::env;
use std::fs;

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V2 Cortex Visualization                         ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // Get image path from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <image_path>", args[0]);
        println!("\nExample:");
        println!("  cargo run --example visualize_v2 --release -- images/input/test_face.png");
        return;
    }

    let image_path = &args[1];
    println!("📷 Loading image: {}\n", image_path);

    // Processing parameters
    let size = 64;
    
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

    // Create visual pathway
    println!("🧠 Initializing visual processing system with V2...");
    let mut pathway = VisualPathway::new(size as usize, size as usize);
    println!("✓ Visual pathway created\n");

    // Process image
    println!("⚡ Processing image through visual pathway...");
    let response = pathway.process_grayscale_image(&image);
    println!("✓ Processing complete\n");

    // Create output directory structure
    let output_dir = "images/output/v2_visualization";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Extract filename without extension
    let input_filename = std::path::Path::new(image_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    println!("💾 Saving visualizations...\n");

    // 1. Save corner map
    let corner_path = format!("{}/{}_corners.png", output_dir, input_filename);
    match visualize_corner_map(&response.v2_features.corner_map, &corner_path) {
        Ok(_) => println!("✓ Corner map saved: {}", corner_path),
        Err(e) => eprintln!("❌ Failed to save corner map: {}", e),
    }

    // 2. Save contours
    let contour_path = format!("{}/{}_contours.png", output_dir, input_filename);
    match visualize_contours(&response.v2_features.contours, size as usize, size as usize, &contour_path) {
        Ok(_) => println!("✓ Contours saved: {}", contour_path),
        Err(e) => eprintln!("❌ Failed to save contours: {}", e),
    }

    // 3. Save composite
    let composite_path = format!("{}/{}_composite.png", output_dir, input_filename);
    match visualize_v2_composite(
        &response.cone_activations,
        &response.v2_features.corner_map,
        &response.v2_features.contours,
        &composite_path
    ) {
        Ok(_) => println!("✓ Composite saved: {}", composite_path),
        Err(e) => eprintln!("❌ Failed to save composite: {}", e),
    }

    println!("\n📊 V2 Statistics:");
    println!("   Corners detected: {}", response.v2_features.corner_count);
    println!("   Contours detected: {}", response.v2_features.contour_count);
    println!("   Total features: {}", response.v2_features.total_features());

    println!("\n🎨 Color Legend:");
    println!("   🔴 Red    = L-junctions (90° corners)");
    println!("   🟢 Green  = T-junctions (occlusions)");
    println!("   🔵 Blue   = X-junctions (crossings)");
    println!("   🟡 Yellow = Y-junctions (3-way)");
    println!("   ⚪ White  = Contours (continuous edges)");

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║   Visualization Complete! ✨                      ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
}
