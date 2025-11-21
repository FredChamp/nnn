use neuron::image_utils::{load_grayscale_image, load_and_resize_grayscale};
use neuron::visual_pathway::VisualPathway;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path> [max_dimension]", args[0]);
        eprintln!("  max_dimension: Maximum width/height (default: 512)");
        std::process::exit(1);
    }

    let image_path = &args[1];
    let max_dim: u32 = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(512);

    println!("🖼️  Loading image: {}", image_path);
    
    // Load image at native resolution first to get dimensions
    let native_image = match load_grayscale_image(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("❌ Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    let native_height = native_image.len();
    let native_width = if native_height > 0 { native_image[0].len() } else { 0 };
    println!("   Native resolution: {}×{} pixels ({} total pixels)", 
             native_width, native_height, native_width * native_height);
    
    // Calculate target dimensions (maintain aspect ratio)
    let (target_width, target_height) = if native_width > max_dim as usize || native_height > max_dim as usize {
        let aspect_ratio = native_width as f32 / native_height as f32;
        if native_width > native_height {
            (max_dim, (max_dim as f32 / aspect_ratio) as u32)
        } else {
            ((max_dim as f32 * aspect_ratio) as u32, max_dim)
        }
    } else {
        (native_width as u32, native_height as u32)
    };
    
    println!("   Processing resolution: {}×{} pixels ({} total pixels)", 
             target_width, target_height, target_width * target_height);
    println!("   Compression: {:.1}× smaller\n", 
             (native_width * native_height) as f32 / (target_width * target_height) as f32);

    // Load and resize if needed
    let image = if target_width != native_width as u32 || target_height != native_height as u32 {
        match load_and_resize_grayscale(image_path, target_width, target_height) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("❌ Error resizing image: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        native_image
    };

    let height = image.len();
    let width = if height > 0 { image[0].len() } else { 0 };

    // Create visual pathway
    println!("🧠 Processing through visual pathway...");
    let mut pathway = VisualPathway::new(width, height);
    let response = pathway.process_grayscale_image(&image);

    println!("\n📊 Visual Cortex Analysis:");
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
            println!("   ├─ {:<12} {:4} │{}│ {:.1}%", 
                     format!("{:?}", shape_type), count, bar, percentage);
        }
    } else {
        println!("      └─ No shapes detected above threshold");
    }

    println!("\n✅ Analysis complete!");
}
