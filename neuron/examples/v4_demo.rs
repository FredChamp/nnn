//! Example: Test V4 cortex shape detection

use neuron::image_utils::load_and_resize_grayscale;
use neuron::visual_pathway::VisualPathway;
use neuron::v4_cortex::ShapeType;
use std::env;

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V4 Cortex Demo: Shape Detection                ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // Get image path from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <image_path>", args[0]);
        println!("\nExample:");
        println!("  cargo run --example v4_demo --release -- images/input/test_face.png");
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
    println!("🧠 Initializing visual processing system with V4...");
    let mut pathway = VisualPathway::new(size as usize, size as usize);
    println!("✓ Visual pathway created\n");

    // Process image
    println!("⚡ Processing image through visual pathway...\n");
    let response = pathway.process_grayscale_image(&image);

    // Display V4 results
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   V4 CORTEX ANALYSIS (SHAPE DETECTION)           ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    println!("🔷 Shape Detection:");
    println!("   Total shapes detected: {}", response.v4_features.shape_count);
    
    if let Some(dominant_shape) = response.v4_features.dominant_shape_type() {
        let shape_name = match dominant_shape {
            ShapeType::Circle => "Circles (smooth closed contours)",
            ShapeType::Rectangle => "Rectangles (4 corners, parallel sides)",
            ShapeType::Triangle => "Triangles (3 corners, 3 sides)",
            ShapeType::Line => "Lines (elongated, minimal corners)",
            ShapeType::Cross => "Crosses (X-junctions, radiating lines)",
            ShapeType::Complex => "Complex/irregular shapes",
        };
        println!("   Dominant shape type: {}", shape_name);
    } else {
        println!("   Dominant shape type: None detected");
    }

    // Count each shape type
    let shape_counts = response.v4_features.count_by_type();

    println!("\n   Shape Type Distribution:");
    println!("   ├─ Circles:    {}", shape_counts.get(&ShapeType::Circle).unwrap_or(&0));
    println!("   ├─ Rectangles: {}", shape_counts.get(&ShapeType::Rectangle).unwrap_or(&0));
    println!("   ├─ Triangles:  {}", shape_counts.get(&ShapeType::Triangle).unwrap_or(&0));
    println!("   ├─ Lines:      {}", shape_counts.get(&ShapeType::Line).unwrap_or(&0));
    println!("   ├─ Crosses:    {}", shape_counts.get(&ShapeType::Cross).unwrap_or(&0));
    println!("   └─ Complex:    {}", shape_counts.get(&ShapeType::Complex).unwrap_or(&0));

    println!("\n📊 Feature Pipeline Summary:");
    println!("   V1 (Orientations): {} active regions", response.features.total_activation);
    println!("   V2 (Corners):      {} corners detected", response.v2_features.corner_count);
    println!("   V2 (Contours):     {} contours detected", response.v2_features.contour_count);
    println!("   V4 (Shapes):       {} shapes detected", response.v4_features.shape_count);

    println!("\n🧠 V4 Interpretation:");
    let total_shapes = response.v4_features.shape_count;
    
    if total_shapes > 20 {
        println!("   • High shape density - complex scene with multiple objects");
    } else if total_shapes > 10 {
        println!("   • Moderate shape density - structured scene");
    } else if total_shapes > 0 {
        println!("   • Low shape density - simple shapes or patterns");
    } else {
        println!("   • No clear shapes detected");
    }

    // Shape-specific interpretation
    let circle_count = shape_counts.get(&ShapeType::Circle).unwrap_or(&0);
    let rect_count = shape_counts.get(&ShapeType::Rectangle).unwrap_or(&0);
    let line_count = shape_counts.get(&ShapeType::Line).unwrap_or(&0);

    if *circle_count > total_shapes / 3 {
        println!("   • Dominated by circular/curved forms");
    }
    if *rect_count > total_shapes / 3 {
        println!("   • Dominated by rectangular/angular structures");
    }
    if *line_count > total_shapes / 2 {
        println!("   • Linear pattern dominant (edges, boundaries)");
    }

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║   V4 Processing Complete! ✨                      ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
}
