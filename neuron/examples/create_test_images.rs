//! Create a test image for visual processing demo

use image::{GrayImage, Luma};

fn main() {
    println!("🎨 Creating test images...\n");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("images/input").expect("Failed to create images/input directory");

    // 1. Simple vertical stripes
    create_vertical_stripes("images/input/test_vertical.png", 128, 128);
    println!("✓ Created: images/input/test_vertical.png (vertical stripes)");

    // 2. Simple horizontal stripes
    create_horizontal_stripes("images/input/test_horizontal.png", 128, 128);
    println!("✓ Created: images/input/test_horizontal.png (horizontal stripes)");

    // 3. Checkerboard pattern
    create_checkerboard("images/input/test_checkerboard.png", 128, 128, 16);
    println!("✓ Created: images/input/test_checkerboard.png (checkerboard)");

    // 4. Diagonal stripes
    create_diagonal_stripes("images/input/test_diagonal.png", 128, 128);
    println!("✓ Created: images/input/test_diagonal.png (diagonal stripes)");

    // 5. Simple face-like pattern
    create_face("images/input/test_face.png", 128, 128);
    println!("✓ Created: images/input/test_face.png (simple face)");

    // 6. Natural scene approximation (texture)
    create_texture("images/input/test_texture.png", 128, 128);
    println!("✓ Created: images/input/test_texture.png (texture pattern)");

    println!("\n✨ Test images created! Use them with:");
    println!("   cargo run --example process_image --release -- images/input/test_vertical.png\n");
}

fn create_vertical_stripes(path: &str, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);
    let stripe_width = width / 8;

    for y in 0..height {
        for x in 0..width {
            let value = if (x / stripe_width) % 2 == 0 { 255 } else { 0 };
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save(path).unwrap();
}

fn create_horizontal_stripes(path: &str, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);
    let stripe_height = height / 8;

    for y in 0..height {
        for x in 0..width {
            let value = if (y / stripe_height) % 2 == 0 { 255 } else { 0 };
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save(path).unwrap();
}

fn create_checkerboard(path: &str, width: u32, height: u32, square_size: u32) {
    let mut img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let value = if ((x / square_size) + (y / square_size)) % 2 == 0 {
                255
            } else {
                0
            };
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save(path).unwrap();
}

fn create_diagonal_stripes(path: &str, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);
    let stripe_width = 16;

    for y in 0..height {
        for x in 0..width {
            let value = if ((x + y) / stripe_width) % 2 == 0 { 255 } else { 0 };
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save(path).unwrap();
}

fn create_face(path: &str, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);
    let cx = width / 2;
    let cy = height / 2;

    // Background
    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, Luma([200]));
        }
    }

    // Face circle (dark)
    for y in 0..height {
        for x in 0..width {
            let dx = x as i32 - cx as i32;
            let dy = y as i32 - cy as i32;
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            
            if dist < width as f32 / 3.0 {
                img.put_pixel(x, y, Luma([100]));
            }
        }
    }

    // Left eye
    draw_circle(&mut img, cx - width / 6, cy - height / 8, width / 15, 0);
    
    // Right eye
    draw_circle(&mut img, cx + width / 6, cy - height / 8, width / 15, 0);

    // Mouth (horizontal line)
    for x in (cx - width / 6)..(cx + width / 6) {
        if x < width {
            img.put_pixel(x, cy + height / 6, Luma([0]));
            img.put_pixel(x, cy + height / 6 + 1, Luma([0]));
        }
    }

    img.save(path).unwrap();
}

fn create_texture(path: &str, width: u32, height: u32) {
    let mut img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Create a pseudo-random texture using sine waves
            let value = (
                (x as f32 * 0.3).sin() * 50.0 +
                (y as f32 * 0.3).cos() * 50.0 +
                ((x + y) as f32 * 0.2).sin() * 50.0 +
                128.0
            ).clamp(0.0, 255.0) as u8;
            
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save(path).unwrap();
}

fn draw_circle(img: &mut GrayImage, cx: u32, cy: u32, radius: u32, value: u8) {
    for y in cy.saturating_sub(radius)..=(cy + radius).min(img.height() - 1) {
        for x in cx.saturating_sub(radius)..=(cx + radius).min(img.width() - 1) {
            let dx = x as i32 - cx as i32;
            let dy = y as i32 - cy as i32;
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            
            if dist < radius as f32 {
                img.put_pixel(x, y, Luma([value]));
            }
        }
    }
}
