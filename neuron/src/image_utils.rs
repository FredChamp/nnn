//! Image loading and preprocessing utilities

use image::{DynamicImage, GenericImageView, ImageReader};
use std::path::Path;

/// Load an image from a file and convert to grayscale matrix
///
/// # Arguments
/// * `path` - Path to the image file
///
/// # Returns
/// A 2D vector of intensity values (0.0 = black, 1.0 = white)
pub fn load_grayscale_image<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<f32>>, String> {
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let gray_img = img.grayscale();
    let (width, height) = gray_img.dimensions();

    let mut matrix = vec![vec![0.0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = gray_img.get_pixel(x, y);
            // Convert to 0.0-1.0 range (normalize)
            matrix[y as usize][x as usize] = pixel[0] as f32 / 255.0;
        }
    }

    Ok(matrix)
}

/// Load an image and resize it to specified dimensions
pub fn load_and_resize_grayscale<P: AsRef<Path>>(
    path: P,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<Vec<f32>>, String> {
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Resize using Lanczos3 filter for better quality
    let resized = img.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );

    let gray_img = resized.grayscale();
    let (width, height) = gray_img.dimensions();

    let mut matrix = vec![vec![0.0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = gray_img.get_pixel(x, y);
            matrix[y as usize][x as usize] = pixel[0] as f32 / 255.0;
        }
    }

    Ok(matrix)
}

/// Save a grayscale matrix as an image file
pub fn save_grayscale_image<P: AsRef<Path>>(
    matrix: &[Vec<f32>],
    path: P,
) -> Result<(), String> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("Empty matrix".to_string());
    }

    let height = matrix.len() as u32;
    let width = matrix[0].len() as u32;

    let mut img_buffer = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let value = (matrix[y as usize][x as usize] * 255.0).clamp(0.0, 255.0) as u8;
            img_buffer.put_pixel(x, y, image::Luma([value]));
        }
    }

    img_buffer
        .save(path)
        .map_err(|e| format!("Failed to save image: {}", e))
}

/// Create a simple ASCII visualization of a grayscale matrix
pub fn ascii_visualization(matrix: &[Vec<f32>], max_width: usize) -> String {
    if matrix.is_empty() {
        return String::new();
    }

    let height = matrix.len();
    let width = matrix[0].len();
    
    // Calculate sampling rate if image is too large
    let sample_x = (width as f32 / max_width as f32).ceil() as usize;
    let sample_y = sample_x;

    let chars = [' ', '░', '▒', '▓', '█'];
    let mut result = String::new();

    for y in (0..height).step_by(sample_y.max(1)) {
        for x in (0..width).step_by(sample_x.max(1)) {
            let value = matrix[y][x];
            let char_idx = (value * (chars.len() - 1) as f32) as usize;
            result.push(chars[char_idx.min(chars.len() - 1)]);
        }
        result.push('\n');
    }

    result
}

/// Get image dimensions from file
pub fn get_image_dimensions<P: AsRef<Path>>(path: P) -> Result<(u32, u32), String> {
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    Ok(img.dimensions())
}

/// Visualize corner map as RGB image
/// Different corner types are shown in different colors
pub fn visualize_corner_map(
    corner_map: &[Vec<Option<crate::v2_cortex::CornerType>>],
    output_path: &str,
) -> Result<(), String> {
    use image::{ImageBuffer, Rgb};
    
    let height = corner_map.len();
    let width = if height > 0 { corner_map[0].len() } else { 0 };
    
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);
    
    for y in 0..height {
        for x in 0..width {
            let color = match corner_map[y][x] {
                Some(crate::v2_cortex::CornerType::LJunction) => Rgb([255u8, 0u8, 0u8]),      // Red
                Some(crate::v2_cortex::CornerType::TJunction) => Rgb([0u8, 255u8, 0u8]),      // Green
                Some(crate::v2_cortex::CornerType::XJunction) => Rgb([0u8, 0u8, 255u8]),      // Blue
                Some(crate::v2_cortex::CornerType::YJunction) => Rgb([255u8, 255u8, 0u8]),    // Yellow
                None => Rgb([0u8, 0u8, 0u8]),                                                  // Black
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    
    img.save(output_path)
        .map_err(|e| format!("Failed to save corner map: {}", e))
}

/// Visualize contours on a black background
pub fn visualize_contours(
    contours: &[Vec<(usize, usize)>],
    width: usize,
    height: usize,
    output_path: &str,
) -> Result<(), String> {
    use image::{ImageBuffer, Rgb};
    
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);
    
    // Fill with black background
    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x as u32, y as u32, Rgb([0u8, 0u8, 0u8]));
        }
    }
    
    // Draw each contour in a different color (cycling through colors)
    let colors = vec![
        Rgb([255u8, 0u8, 0u8]),      // Red
        Rgb([0u8, 255u8, 0u8]),      // Green
        Rgb([0u8, 0u8, 255u8]),      // Blue
        Rgb([255u8, 255u8, 0u8]),    // Yellow
        Rgb([255u8, 0u8, 255u8]),    // Magenta
        Rgb([0u8, 255u8, 255u8]),    // Cyan
        Rgb([255u8, 128u8, 0u8]),    // Orange
        Rgb([128u8, 0u8, 255u8]),    // Purple
    ];
    
    for (i, contour) in contours.iter().enumerate() {
        let color = colors[i % colors.len()];
        for &(x, y) in contour {
            if x < width && y < height {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
    
    img.save(output_path)
        .map_err(|e| format!("Failed to save contours: {}", e))
}

/// Create a composite visualization with original image, corners, and contours
pub fn visualize_v2_composite(
    original: &[Vec<f32>],
    corner_map: &[Vec<Option<crate::v2_cortex::CornerType>>],
    contours: &[Vec<(usize, usize)>],
    output_path: &str,
) -> Result<(), String> {
    use image::{ImageBuffer, Rgb};
    
    let height = original.len();
    let width = if height > 0 { original[0].len() } else { 0 };
    
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);
    
    // Start with grayscale original image
    for y in 0..height {
        for x in 0..width {
            let gray = (original[y][x] * 255.0) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([gray, gray, gray]));
        }
    }
    
    // Overlay contours in white (semi-transparent effect via brightening)
    for contour in contours {
        for &(x, y) in contour {
            if x < width && y < height {
                img.put_pixel(x as u32, y as u32, Rgb([255u8, 255u8, 255u8]));
            }
        }
    }
    
    // Overlay corners in bright colors (highest priority)
    for y in 0..height {
        for x in 0..width {
            if let Some(corner_type) = corner_map[y][x] {
                let color = match corner_type {
                    crate::v2_cortex::CornerType::LJunction => Rgb([255u8, 0u8, 0u8]),      // Red
                    crate::v2_cortex::CornerType::TJunction => Rgb([0u8, 255u8, 0u8]),      // Green
                    crate::v2_cortex::CornerType::XJunction => Rgb([0u8, 0u8, 255u8]),      // Blue
                    crate::v2_cortex::CornerType::YJunction => Rgb([255u8, 255u8, 0u8]),    // Yellow
                };
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
    
    img.save(output_path)
        .map_err(|e| format!("Failed to save composite: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_visualization() {
        let matrix = vec![
            vec![0.0, 0.5, 1.0],
            vec![0.25, 0.75, 0.5],
        ];

        let viz = ascii_visualization(&matrix, 10);
        assert!(!viz.is_empty());
        assert!(viz.contains('\n'));
    }
}
