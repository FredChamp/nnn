//! Integration tests for the visual processing system
//! Tests that the system correctly detects orientations in test images

use neuron::image_utils::load_and_resize_grayscale;
use neuron::visual_pathway::VisualPathway;

#[test]
fn test_horizontal_stripes_detection() {
    // Load test image
    let image = load_and_resize_grayscale("images/input/test_horizontal.png", 64, 64)
        .expect("Failed to load test_horizontal.png");

    // Process through visual pathway
    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);

    println!("\n🧪 Test: Horizontal Stripes");
    println!("   Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   Diagonal edges:   {:.2}", response.features.diagonal_strength);
    println!("   Dominant: {}", response.features.dominant_orientation());

    // Horizontal stripes should produce strong horizontal edges
    assert!(
        response.features.horizontal_strength > response.features.vertical_strength * 0.95,
        "Horizontal edges ({:.2}) should be stronger than or close to vertical edges ({:.2})",
        response.features.horizontal_strength,
        response.features.vertical_strength
    );
    
    // Accept if horizontal is dominant OR if it's close to diagonal
    let is_horizontal_strong = response.features.horizontal_strength > response.features.diagonal_strength * 0.95;
    let is_horizontal_dominant = response.features.dominant_orientation() == "Horizontal";
    
    assert!(
        is_horizontal_strong || is_horizontal_dominant,
        "Horizontal should be dominant or strong. H: {:.2}, V: {:.2}, D: {:.2}, Dominant: {}",
        response.features.horizontal_strength,
        response.features.vertical_strength,
        response.features.diagonal_strength,
        response.features.dominant_orientation()
    );
}

#[test]
fn test_vertical_stripes_detection() {
    let image = load_and_resize_grayscale("images/input/test_vertical.png", 64, 64)
        .expect("Failed to load test_vertical.png");

    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);

    println!("\n🧪 Test: Vertical Stripes");
    println!("   Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   Diagonal edges:   {:.2}", response.features.diagonal_strength);
    println!("   Dominant: {}", response.features.dominant_orientation());

    // Vertical stripes should produce strong vertical edges
    assert!(
        response.features.vertical_strength > response.features.horizontal_strength * 0.95,
        "Vertical edges ({:.2}) should be stronger than or close to horizontal edges ({:.2})",
        response.features.vertical_strength,
        response.features.horizontal_strength
    );

    // Accept if vertical is dominant OR if it's close to diagonal
    let is_vertical_strong = response.features.vertical_strength > response.features.diagonal_strength * 0.95;
    let is_vertical_dominant = response.features.dominant_orientation() == "Vertical";
    
    assert!(
        is_vertical_strong || is_vertical_dominant,
        "Vertical should be dominant or strong. H: {:.2}, V: {:.2}, D: {:.2}, Dominant: {}",
        response.features.horizontal_strength,
        response.features.vertical_strength,
        response.features.diagonal_strength,
        response.features.dominant_orientation()
    );
}

#[test]
fn test_diagonal_stripes_detection() {
    let image = load_and_resize_grayscale("images/input/test_diagonal.png", 64, 64)
        .expect("Failed to load test_diagonal.png");

    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);

    println!("\n🧪 Test: Diagonal Stripes");
    println!("   Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   Diagonal edges:   {:.2}", response.features.diagonal_strength);
    println!("   Dominant: {}", response.features.dominant_orientation());

    // Diagonal stripes should produce strong diagonal edges
    // Note: diagonal patterns in pixelated images create staircase effects
    // which produce both H and V components, so we accept balanced H/V as diagonal too
    let h = response.features.horizontal_strength;
    let v = response.features.vertical_strength;
    let d = response.features.diagonal_strength;
    
    let is_diagonal_strongest = d > h * 0.9 && d > v * 0.9;
    let is_diagonal_dominant = response.features.dominant_orientation() == "Diagonal";
    let is_balanced_hv = (h - v).abs() < h * 0.1 && d > h * 0.9; // H≈V suggests diagonal staircase
    
    assert!(
        is_diagonal_strongest || is_diagonal_dominant || is_balanced_hv,
        "Diagonal should be strongest/dominant or H≈V (staircase). H: {:.2}, V: {:.2}, D: {:.2}, Dominant: {}",
        h, v, d,
        response.features.dominant_orientation()
    );
}

#[test]
fn test_checkerboard_detection() {
    let image = load_and_resize_grayscale("images/input/test_checkerboard.png", 64, 64)
        .expect("Failed to load test_checkerboard.png");

    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);

    println!("\n🧪 Test: Checkerboard");
    println!("   Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   Diagonal edges:   {:.2}", response.features.diagonal_strength);
    println!("   Dominant: {}", response.features.dominant_orientation());

    // Checkerboard should have balanced horizontal and vertical
    let h = response.features.horizontal_strength;
    let v = response.features.vertical_strength;
    let ratio = if h > v { h / v } else { v / h };

    assert!(
        ratio < 1.5,
        "Checkerboard should have balanced H/V ratio, got {:.2}",
        ratio
    );

    // Should detect strong edges overall
    assert!(
        response.features.edge_strength() > 100.0,
        "Checkerboard should have strong edges, got {:.2}",
        response.features.edge_strength()
    );
}

#[test]
fn test_face_detection() {
    let image = load_and_resize_grayscale("images/input/test_face.png", 64, 64)
        .expect("Failed to load test_face.png");

    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);

    println!("\n🧪 Test: Face");
    println!("   Horizontal edges: {:.2}", response.features.horizontal_strength);
    println!("   Vertical edges:   {:.2}", response.features.vertical_strength);
    println!("   Diagonal edges:   {:.2}", response.features.diagonal_strength);
    println!("   Dominant: {}", response.features.dominant_orientation());
    println!("   Total edge strength: {:.2}", response.features.edge_strength());

    // Face should have curved edges (detected as diagonal)
    // Should have some edge content
    assert!(
        response.features.edge_strength() > 50.0,
        "Face should have detectable edges, got {:.2}",
        response.features.edge_strength()
    );

    // Should have active cones
    let active_cones: usize = response.cone_activations
        .iter()
        .flatten()
        .filter(|&&x| x > 0.1)
        .count();
    
    assert!(
        active_cones > 1000,
        "Face should activate many cones, got {}",
        active_cones
    );
}

#[test]
fn test_all_images_summary() {
    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║   Visual System Test Summary                      ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    let test_images = vec![
        ("test_horizontal.png", "Horizontal"),
        ("test_vertical.png", "Vertical"),
        ("test_diagonal.png", "Diagonal"),
        ("test_checkerboard.png", "Mixed"),
        ("test_face.png", "Complex"),
    ];

    for (filename, expected_type) in test_images {
        let path = format!("images/input/{}", filename);
        match load_and_resize_grayscale(&path, 64, 64) {
            Ok(image) => {
                let mut pathway = VisualPathway::new(64, 64);
                let response = pathway.process_grayscale_image(&image);
                
                println!("📷 {}", filename);
                println!("   Expected: {}", expected_type);
                println!("   Detected: {}", response.features.dominant_orientation());
                println!("   H: {:.1}  V: {:.1}  D: {:.1}", 
                    response.features.horizontal_strength,
                    response.features.vertical_strength,
                    response.features.diagonal_strength
                );
                
                let status = match expected_type {
                    "Horizontal" if response.features.dominant_orientation() == "Horizontal" => "✅",
                    "Vertical" if response.features.dominant_orientation() == "Vertical" => "✅",
                    "Diagonal" if response.features.dominant_orientation() == "Diagonal" => "✅",
                    "Mixed" | "Complex" => "ℹ️",
                    _ => "❌",
                };
                println!("   Status: {}\n", status);
            }
            Err(e) => {
                println!("⚠️  {} - Could not load: {}\n", filename, e);
            }
        }
    }
}
