//! Example: Process image through V2 cortex to detect corners and contours

use neuron::image_utils::{ascii_visualization, load_and_resize_grayscale};
use neuron::visual_pathway::VisualPathway;
use std::env;

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V2 Cortex Demo: Corners & Contours            ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // Get image path from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <image_path>", args[0]);
        println!("\nExample:");
        println!("  cargo run --example v2_demo --release -- images/input/test_face.png");
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

    // Show ASCII preview
    println!("\n📊 Input Image Preview (ASCII):");
    println!("{}", ascii_visualization(&image, 60));

    // Create visual pathway
    println!("🧠 Initializing visual processing system with V2...");
    let mut pathway = VisualPathway::new(size as usize, size as usize);
    println!("✓ Visual pathway created\n");

    // Process image
    println!("⚡ Processing image through visual pathway...\n");
    let response = pathway.process_grayscale_image(&image);

    // Display V2 results
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V2 CORTEX ANALYSIS                              ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    println!("🔺 Corner Detection (Junctions):");
    println!("   Total corners detected: {}", response.v2_features.corner_count);
    
    if let Some(dominant_corner) = response.v2_features.dominant_corner_type() {
        let corner_name = match dominant_corner {
            neuron::v2_cortex::CornerType::LJunction => "L-junctions (90° corners)",
            neuron::v2_cortex::CornerType::TJunction => "T-junctions (occlusions)",
            neuron::v2_cortex::CornerType::XJunction => "X-junctions (crossings)",
            neuron::v2_cortex::CornerType::YJunction => "Y-junctions (3-way)",
        };
        println!("   Dominant corner type: {}", corner_name);
    } else {
        println!("   Dominant corner type: None detected");
    }

    // Count each corner type
    let mut l_count = 0;
    let mut t_count = 0;
    let mut x_count = 0;
    let mut y_count = 0;

    for row in &response.v2_features.corner_map {
        for corner_opt in row {
            if let Some(corner) = corner_opt {
                match corner {
                    neuron::v2_cortex::CornerType::LJunction => l_count += 1,
                    neuron::v2_cortex::CornerType::TJunction => t_count += 1,
                    neuron::v2_cortex::CornerType::XJunction => x_count += 1,
                    neuron::v2_cortex::CornerType::YJunction => y_count += 1,
                }
            }
        }
    }

    println!("\n   Corner Type Distribution:");
    println!("   ├─ L-junctions (corners):  {}", l_count);
    println!("   ├─ T-junctions (occlusions): {}", t_count);
    println!("   ├─ X-junctions (crossings): {}", x_count);
    println!("   └─ Y-junctions (3-way):    {}", y_count);

    println!("\n📐 Contour Detection (Continuous edges):");
    println!("   Total contours found: {}", response.v2_features.contour_count);
    
    if !response.v2_features.contours.is_empty() {
        let lengths: Vec<usize> = response.v2_features.contours.iter().map(|c| c.len()).collect();
        let max_length = lengths.iter().max().unwrap_or(&0);
        let avg_length: f32 = lengths.iter().sum::<usize>() as f32 / lengths.len() as f32;
        
        println!("   Longest contour: {} pixels", max_length);
        println!("   Average contour length: {:.1} pixels", avg_length);

        // Show some contour examples
        println!("\n   Contour Examples:");
        for (i, contour) in response.v2_features.contours.iter().take(3).enumerate() {
            println!("   Contour {}: {} pixels long", i + 1, contour.len());
            if contour.len() >= 2 {
                let start = contour[0];
                let end = contour[contour.len() - 1];
                println!("      From ({}, {}) to ({}, {})", start.0, start.1, end.0, end.1);
            }
        }
    }

    println!("\n🔬 V2 Feature Summary:");
    println!("   Total features: {} (corners + contours)", response.v2_features.total_features());
    
    // Interpretation
    println!("\n🧠 V2 Interpretation:");
    if response.v2_features.corner_count > 50 {
        println!("   • High corner density - geometric/architectural structures");
    } else if response.v2_features.corner_count > 20 {
        println!("   • Moderate corner density - structured objects");
    } else {
        println!("   • Low corner density - smooth/curved objects");
    }

    if response.v2_features.contour_count > 10 {
        println!("   • Many contours - complex object boundaries");
    } else if response.v2_features.contour_count > 5 {
        println!("   • Moderate contours - distinct object shapes");
    } else {
        println!("   • Few contours - simple or fragmented shapes");
    }

    if l_count > t_count * 2 {
        println!("   • Dominated by L-junctions - rectangular/box-like structures");
    }

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║   V2 Processing Complete! ✨                      ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
}
