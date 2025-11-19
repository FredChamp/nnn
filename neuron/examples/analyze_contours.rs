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
    println!("🔍 Analyzing contours in: {}\n", image_path);
    
    let image = match load_grayscale_image(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    };

    let height = image.len();
    let width = if height > 0 { image[0].len() } else { 0 };
    
    let mut pathway = VisualPathway::new(width, height);
    let response = pathway.process_grayscale_image(&image);

    println!("📊 Contour Analysis:");
    println!("   Total contours: {}", response.v2_features.contours.len());
    
    // Analyze contour lengths
    let mut lengths: Vec<usize> = response.v2_features.contours
        .iter()
        .map(|c| c.len())
        .collect();
    lengths.sort_unstable();
    
    if lengths.is_empty() {
        println!("   No contours found!");
        return;
    }
    
    let total_pixels: usize = lengths.iter().sum();
    let min = lengths[0];
    let max = lengths[lengths.len() - 1];
    let median = lengths[lengths.len() / 2];
    let mean = total_pixels / lengths.len();
    
    println!("\n   Length Statistics:");
    println!("   ├─ Min:    {} pixels", min);
    println!("   ├─ Max:    {} pixels", max);
    println!("   ├─ Median: {} pixels", median);
    println!("   ├─ Mean:   {} pixels", mean);
    println!("   └─ Total:  {} pixels\n", total_pixels);
    
    // Distribution by length buckets
    let mut short = 0;  // 1-5 pixels
    let mut medium = 0; // 6-15 pixels
    let mut long = 0;   // 16-30 pixels
    let mut very_long = 0; // 31+ pixels
    
    for &len in &lengths {
        if len <= 5 {
            short += 1;
        } else if len <= 15 {
            medium += 1;
        } else if len <= 30 {
            long += 1;
        } else {
            very_long += 1;
        }
    }
    
    println!("   Distribution by Length:");
    println!("   ├─ Short (1-5px):      {} ({:.1}%)", short, short as f32 / lengths.len() as f32 * 100.0);
    println!("   ├─ Medium (6-15px):    {} ({:.1}%)", medium, medium as f32 / lengths.len() as f32 * 100.0);
    println!("   ├─ Long (16-30px):     {} ({:.1}%)", long, long as f32 / lengths.len() as f32 * 100.0);
    println!("   └─ Very Long (31+px):  {} ({:.1}%)", very_long, very_long as f32 / lengths.len() as f32 * 100.0);
    
    // Show top 10 longest contours
    lengths.reverse();
    println!("\n   Top 10 Longest Contours:");
    for (i, &len) in lengths.iter().take(10).enumerate() {
        println!("   {}. {} pixels", i + 1, len);
    }
}
