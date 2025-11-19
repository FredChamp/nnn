/// Create a test image with a clear circle
use image::{GrayImage, Luma};

fn main() {
    let width = 128;
    let height = 128;
    let mut img = GrayImage::new(width, height);
    
    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Luma([255u8]);
    }
    
    // Draw a black circle (center at 64,64, radius 30)
    let center_x = 64.0;
    let center_y = 64.0;
    let radius = 30.0;
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            // Draw circle outline (thickness 2 pixels)
            if (dist - radius).abs() < 2.0 {
                img.put_pixel(x, y, Luma([0u8]));
            }
        }
    }
    
    img.save("images/input/test_circle.png").unwrap();
    println!("✅ Created test_circle.png (128×128 with circle radius 30)");
}
