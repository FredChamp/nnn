use neuron::image_utils::load_grayscale_image;
use neuron::visual_pathway::VisualPathway;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];
    println!("🖼️  Loading image: {}", image_path);
    
    // Load image at NATIVE RESOLUTION (no resize)
    let image = match load_grayscale_image(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("❌ Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    let height = image.len();
    let width = if height > 0 { image[0].len() } else { 0 };
    println!("   Resolution: {}×{} pixels ({} total pixels)\n", width, height, width * height);

    // Create visual pathway with NATIVE dimensions
    let mut pathway = VisualPathway::new(width, height);

    println!("🧠 Processing through visual pathway...");
    let response = pathway.process_grayscale_image(&image);

    println!("\n📊 Visual Cortex Analysis (Native Resolution):");
    println!("   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // V1 Response
    println!("   🔹 V1 (Primary Visual Cortex):");
    let total_v1: f32 = response.orientation_map.iter()
        .flatten()
        .filter(|o| o.is_some())
        .count() as f32;
    println!("      └─ {} orientation-selective neurons active", total_v1 as usize);
    
    // V2 Response
    println!("\n   🔹 V2 (Secondary Visual Cortex):");
    println!("      ├─ Corners detected: {}", response.v2_features.corner_count);
    
    // Count corners by type
    let mut l_count = 0;
    let mut t_count = 0;
    let mut x_count = 0;
    let mut y_count = 0;
    for row in &response.v2_features.corner_map {
        for corner_opt in row {
            if let Some(corner) = corner_opt {
                match corner {
                    neuron::CornerType::LJunction => l_count += 1,
                    neuron::CornerType::TJunction => t_count += 1,
                    neuron::CornerType::XJunction => x_count += 1,
                    neuron::CornerType::YJunction => y_count += 1,
                }
            }
        }
    }
    if l_count > 0 { println!("      │  ├─ L-junctions: {}", l_count); }
    if t_count > 0 { println!("      │  ├─ T-junctions: {}", t_count); }
    if x_count > 0 { println!("      │  ├─ X-junctions: {}", x_count); }
    if y_count > 0 { println!("      │  ├─ Y-junctions: {}", y_count); }
    println!("      └─ Contours traced: {}", response.v2_features.contours.len());
    
    // V4 Response
    println!("\n   🔹 V4 (Shape Detection Cortex):");
    let shape_count = response.v4_features.shape_count;
    let counts = response.v4_features.count_by_type();
    println!("      └─ Total shapes: {}", shape_count);
    
    if shape_count > 0 {
        let dominant = response.v4_features.dominant_shape_type();
        if let Some(dom_shape) = dominant {
            let description = match dom_shape {
                neuron::ShapeType::Circle => "smooth closed curves",
                neuron::ShapeType::Rectangle => "four-sided enclosures",
                neuron::ShapeType::Triangle => "three-sided polygons",
                neuron::ShapeType::Line => "elongated straight segments",
                neuron::ShapeType::Cross => "X-junctions and perpendicular intersections",
                neuron::ShapeType::Complex => "irregular multi-feature forms",
            };
            
            println!("\n   🎯 Dominant Shape: {:?} ({})", dom_shape, description);
        }
        println!("   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Shape distribution
        println!("\n   Shape Distribution:");
        for (shape_type, count) in counts {
            let percentage = (*count as f32 / shape_count as f32) * 100.0;
            let bar_length = (percentage / 2.0) as usize;
            let bar = "█".repeat(bar_length);
            println!("   ├─ {:<12} {:3} │{}│ {:.1}%", 
                     format!("{:?}", shape_type), count, bar, percentage);
        }
    } else {
        println!("      └─ No shapes detected above threshold");
    }

    println!("\n✅ Analysis complete!");
}
