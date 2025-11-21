use neuron::image_utils::{load_grayscale_image, load_and_resize_grayscale, visualize_v4_shapes, visualize_v4_with_legend};
use neuron::visual_pathway::VisualPathway;
use std::env;
use std::fs;

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
    
    // Load and potentially resize
    let native_image = match load_grayscale_image(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("❌ Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    let native_height = native_image.len();
    let native_width = if native_height > 0 { native_image[0].len() } else { 0 };
    
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
    
    println!("   Native: {}×{}, Processing: {}×{}", 
             native_width, native_height, target_width, target_height);

    let image = if target_width != native_width as u32 || target_height != native_height as u32 {
        match load_and_resize_grayscale(image_path, target_width, target_height) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("❌ Error resizing: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        native_image
    };

    // Process through visual pathway
    println!("\n🧠 Processing through visual pathway...");
    let mut pathway = VisualPathway::new(target_width as usize, target_height as usize);
    let response = pathway.process_grayscale_image(&image);

    // Create output directory
    fs::create_dir_all("images/output/v4_visualization").ok();
    
    // Extract filename without extension
    let filename = std::path::Path::new(image_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    // Save V4 shape visualization
    let output_basic = format!("images/output/v4_visualization/{}_v4_shapes.png", filename);
    println!("\n💾 Saving V4 shape map...");
    match visualize_v4_shapes(&image, &response.v4_features.shape_map, &output_basic) {
        Ok(_) => println!("   ✅ Saved: {}", output_basic),
        Err(e) => eprintln!("   ❌ Error: {}", e),
    }
    
    // Save V4 with legend
    let output_legend = format!("images/output/v4_visualization/{}_v4_with_legend.png", filename);
    println!("💾 Saving V4 with legend...");
    match visualize_v4_with_legend(&image, &response.v4_features.shape_map, &output_legend) {
        Ok(_) => println!("   ✅ Saved: {}", output_legend),
        Err(e) => eprintln!("   ❌ Error: {}", e),
    }

    // Print statistics
    println!("\n📊 V4 Shape Detection Statistics:");
    println!("   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let shape_count = response.v4_features.shape_count;
    let counts = response.v4_features.count_by_type();
    
    println!("   Total shapes detected: {}", shape_count);
    
    if shape_count > 0 {
        println!("\n   Shape Distribution:");
        
        // Sort by count descending
        let mut sorted_counts: Vec<_> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));
        
        for (shape_type, count) in sorted_counts {
            let percentage = (*count as f32 / shape_count as f32) * 100.0;
            let bar_length = (percentage / 2.0) as usize;
            let bar = "█".repeat(bar_length);
            
            let color_legend = match shape_type {
                neuron::ShapeType::Circle => "🟣 Magenta",
                neuron::ShapeType::Rectangle => "🟢 Green",
                neuron::ShapeType::Triangle => "🟡 Yellow",
                neuron::ShapeType::Line => "🔵 Cyan",
                neuron::ShapeType::Cross => "🔴 Red",
                neuron::ShapeType::Complex => "⚫ Gray",
            };
            
            println!("   {:12} {:4} │{}│ {:5.1}%  {}", 
                     format!("{:?}", shape_type), count, bar, percentage, color_legend);
        }
        
        if let Some(dominant) = response.v4_features.dominant_shape_type() {
            println!("\n   🎯 Dominant: {:?}", dominant);
        }
    }
    
    println!("\n✅ Visualization complete!");
    println!("\n   Color Legend:");
    println!("   ━━━━━━━━━━━━━━━━");
    println!("   🟣 Circle    - Smooth closed curves");
    println!("   🟢 Rectangle - Four-sided enclosures");
    println!("   🟡 Triangle  - Three-sided polygons");
    println!("   🔵 Line      - Elongated segments");
    println!("   🔴 Cross     - X-junctions");
    println!("   ⚫ Complex   - Irregular forms");
}
