//! Example: Batch process all test images with V2 visualization

use neuron::image_utils::{load_and_resize_grayscale, visualize_v2_composite};
use neuron::visual_pathway::VisualPathway;
use std::fs;

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V2 Batch Visualization                          ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // List of test images
    let test_images = vec![
        "images/input/test_face.png",
        "images/input/test_checkerboard.png",
        "images/input/test_horizontal.png",
        "images/input/test_vertical.png",
        "images/input/test_diagonal.png",
    ];

    // Create output directory
    let output_dir = "images/output/v2_batch";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let size = 64;
    let mut pathway = VisualPathway::new(size, size);
    
    println!("🧠 Visual pathway initialized ({}x{})\n", size, size);

    for (i, image_path) in test_images.iter().enumerate() {
        println!("┌─────────────────────────────────────────────────┐");
        println!("│ [{}/{}] Processing: {}", i + 1, test_images.len(), image_path);
        println!("└─────────────────────────────────────────────────┘");

        // Check if file exists
        if !std::path::Path::new(image_path).exists() {
            println!("⚠️  File not found, skipping...\n");
            continue;
        }

        // Load image
        let image = match load_and_resize_grayscale(image_path, size as u32, size as u32) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("❌ Failed to load: {}\n", e);
                continue;
            }
        };

        // Process through V2
        let response = pathway.process_grayscale_image(&image);

        // Extract filename
        let filename = std::path::Path::new(image_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");

        // Save composite visualization
        let output_path = format!("{}/{}_v2.png", output_dir, filename);
        match visualize_v2_composite(
            &response.cone_activations,
            &response.v2_features.corner_map,
            &response.v2_features.contours,
            &output_path,
        ) {
            Ok(_) => {
                println!("✓ Saved: {}", output_path);
                println!("  └─ Corners: {} | Contours: {} | Total features: {}",
                    response.v2_features.corner_count,
                    response.v2_features.contour_count,
                    response.v2_features.total_features()
                );
            }
            Err(e) => eprintln!("❌ Failed to save: {}", e),
        }

        println!();
    }

    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   Batch Processing Complete! ✨                   ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!("\n📁 Output directory: {}", output_dir);
    println!("\n🎨 Color Legend:");
    println!("   🔴 Red    = L-junctions (90° corners)");
    println!("   🟢 Green  = T-junctions (occlusions)");
    println!("   🔵 Blue   = X-junctions (crossings)");
    println!("   🟡 Yellow = Y-junctions (3-way)");
    println!("   ⚪ White  = Contours (continuous edges)");
}
