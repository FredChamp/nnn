//! V4 Visual Cortex - Shape and Object Detection
//!
//! V4 is a higher-level visual area that processes complex shapes by combining
//! information from V2 (corners, junctions, contours). V4 neurons are selective
//! for specific shapes and can recognize objects invariant to size and position.
//!
//! Key features:
//! - Shape detection (circles, rectangles, triangles, etc.)
//! - Curvature analysis
//! - Intermediate complexity features
//! - Size and position invariance

use crate::v2_cortex::{CornerType, V2Response};

/// Types of shapes that V4 can detect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeType {
    Circle,
    Rectangle,
    Triangle,
    Line,
    Cross,
    Complex,
}

/// V4 shape detector neuron
#[derive(Debug)]
pub struct V4ShapeDetector {
    _id: usize,
    x: usize,
    y: usize,
    shape_type: ShapeType,
    receptive_field_size: usize,
    activation: f32,
}

impl V4ShapeDetector {
    /// Creates a new V4 shape detector
    pub fn new(
        id: usize,
        x: usize,
        y: usize,
        shape_type: ShapeType,
        rf_size: usize,
    ) -> Self {
        Self {
            _id: id,
            x,
            y,
            shape_type,
            receptive_field_size: rf_size,
            activation: 0.0,
        }
    }

    /// Compute response to V2 features (corners, contours)
    pub fn compute_response(&mut self, v2_response: &V2Response) {
        let response = match self.shape_type {
            ShapeType::Circle => self.detect_circle(v2_response),
            ShapeType::Rectangle => self.detect_rectangle(v2_response),
            ShapeType::Triangle => self.detect_triangle(v2_response),
            ShapeType::Line => self.detect_line(v2_response),
            ShapeType::Cross => self.detect_cross(v2_response),
            ShapeType::Complex => self.detect_complex(v2_response),
        };

        self.activation = response;
    }

    /// Detect circular shapes (smooth contours, no corners)
    fn detect_circle(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut contour_pixels = 0;
        let mut corner_count = 0;
        let mut longest_contour = 0;

        // Count contour pixels and find longest contour in receptive field
        for contour in &v2_response.contours {
            let local_pixels: Vec<_> = contour.iter()
                .filter(|&&(x, y)| self.in_receptive_field(x, y, rf))
                .collect();
            
            let local_count = local_pixels.len();
            contour_pixels += local_count;
            if local_count > longest_contour {
                longest_contour = local_count;
            }
        }

        // Count corners (circles should have few/no corners)
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                if v2_response.corner_map[y][x].is_some() {
                    corner_count += 1;
                }
            }
        }

        // Circle: Many small curved contour fragments forming a circular pattern
        // Real circles get fragmented into many small contours by V2
        // Key insight: circles have SMOOTH curves (few corners relative to contour pixels)
        //              lines/grids have MANY corners (intersections, angles)
        
        let contour_density = contour_pixels as f32 / (rf * rf) as f32;
        let corner_to_contour_ratio = if contour_pixels > 0 {
            corner_count as f32 / contour_pixels as f32
        } else {
            1.0
        };
        
        // STRICT circle criteria: high contour density BUT very low corner ratio
        // Circles should be smooth (corner_ratio < 0.08 means less than 8% corners)
        if contour_pixels >= 20 && corner_to_contour_ratio < 0.08 && contour_density > 0.08 {
            // Many small smooth fragments with very few corners = circle
            let smoothness_score = (1.0 - corner_to_contour_ratio * 10.0) * 25.0;
            let density_bonus = if contour_density > 0.15 { 5.0 } else { 0.0 };
            (smoothness_score + density_bonus).max(10.0).min(25.0)
        } else if longest_contour >= 6 && corner_count <= 3 && contour_pixels < 35 {
            // Fallback: single long smooth contour (for small circles)
            let continuity = longest_contour as f32 / contour_pixels.max(1) as f32;
            if continuity > 0.3 {
                (longest_contour as f32 * 1.5).min(18.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Detect rectangular shapes (4 L-junctions or corners, parallel contours)
    fn detect_rectangle(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut l_junction_count = 0;
        let mut x_junction_count = 0;
        let mut contour_segments = 0;

        // Count L-junctions (4 corners of rectangle)
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                match v2_response.corner_map[y][x] {
                    Some(CornerType::LJunction) => l_junction_count += 1,
                    Some(CornerType::XJunction) => x_junction_count += 1,
                    _ => {}
                }
            }
        }

        // Count contour segments
        for contour in &v2_response.contours {
            if contour.iter().any(|&(x, y)| self.in_receptive_field(x, y, rf)) {
                contour_segments += 1;
            }
        }

        // Rectangle: 3-5 L-junctions (corners) + some straight contours
        if l_junction_count >= 3 && contour_segments >= 3 {
            ((l_junction_count + contour_segments) as f32 * 1.5).min(25.0)
        } else if x_junction_count >= 2 && contour_segments >= 2 {
            // Alternative: X-junctions from overlapping rectangles
            ((x_junction_count + contour_segments) as f32).min(20.0)
        } else {
            0.0
        }
    }

    /// Detect triangular shapes (3 corners, 3 sides)
    fn detect_triangle(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut l_junction_count = 0;
        let mut y_junction_count = 0;
        let mut contour_segments = 0;

        // Count junctions
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                match v2_response.corner_map[y][x] {
                    Some(CornerType::LJunction) => l_junction_count += 1,
                    Some(CornerType::YJunction) => y_junction_count += 1,
                    _ => {}
                }
            }
        }

        // Count contours
        for contour in &v2_response.contours {
            if contour.iter().any(|&(x, y)| self.in_receptive_field(x, y, rf)) {
                contour_segments += 1;
            }
        }

        // Triangle: 3 corners (L or Y junctions) + 3 contour segments
        let total_corners = l_junction_count + y_junction_count;
        if total_corners == 3 && contour_segments >= 3 {
            ((total_corners + contour_segments) as f32 * 2.0).min(20.0)
        } else if total_corners >= 2 && total_corners <= 4 && contour_segments >= 2 {
            // Approximate triangle
            ((total_corners + contour_segments) as f32).min(15.0)
        } else {
            0.0
        }
    }

    /// Detect line shapes (long contours, few corners)
    fn detect_line(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut longest_contour = 0;
        let mut corner_count = 0;

        // Find longest contour in receptive field
        for contour in &v2_response.contours {
            let local_pixels: Vec<_> = contour.iter()
                .filter(|&&(x, y)| self.in_receptive_field(x, y, rf))
                .collect();
            
            if local_pixels.len() > longest_contour {
                longest_contour = local_pixels.len();
            }
        }

        // Count corners
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                if v2_response.corner_map[y][x].is_some() {
                    corner_count += 1;
                }
            }
        }

        // Line: long contour, minimal corners
        // For real images: lines can have some corners (intersections, slight bends)
        if longest_contour > 8 && corner_count <= 4 {
            // Longer lines get higher activation even with more corners
            let base_score = longest_contour as f32 * 0.9;
            // Penalize corners but don't eliminate
            let corner_penalty = corner_count as f32 * 0.5;
            (base_score - corner_penalty).max(5.0).min(20.0)
        } else if longest_contour > 5 && corner_count <= 2 {
            // Short clean lines
            (longest_contour as f32 * 0.7).min(15.0)
        } else {
            0.0
        }
    }

    /// Detect cross shapes (X-junction + 4 radiating lines)
    fn detect_cross(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut x_junction_count = 0;
        let mut t_junction_count = 0;
        let mut contour_count = 0;

        // Count X and T junctions
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                match v2_response.corner_map[y][x] {
                    Some(CornerType::XJunction) => x_junction_count += 1,
                    Some(CornerType::TJunction) => t_junction_count += 1,
                    _ => {}
                }
            }
        }

        // Count contours
        for contour in &v2_response.contours {
            if contour.iter().any(|&(x, y)| self.in_receptive_field(x, y, rf)) {
                contour_count += 1;
            }
        }

        // Cross: X-junctions + multiple radiating contours
        if x_junction_count >= 1 && contour_count >= 3 {
            ((x_junction_count * 5 + contour_count) as f32).min(25.0)
        } else if t_junction_count >= 2 && contour_count >= 3 {
            // T-junctions can approximate a cross
            ((t_junction_count * 3 + contour_count) as f32).min(20.0)
        } else {
            0.0
        }
    }

    /// Detect complex/irregular shapes
    fn detect_complex(&self, v2_response: &V2Response) -> f32 {
        let rf = self.receptive_field_size;
        let mut total_corners = 0;
        let mut contour_count = 0;

        // Count all corners
        for y in self.y.saturating_sub(rf)..=(self.y + rf).min(v2_response.corner_map.len() - 1) {
            for x in self.x.saturating_sub(rf)..=(self.x + rf).min(v2_response.corner_map[0].len() - 1) {
                if v2_response.corner_map[y][x].is_some() {
                    total_corners += 1;
                }
            }
        }

        // Count contours
        for contour in &v2_response.contours {
            if contour.iter().any(|&(x, y)| self.in_receptive_field(x, y, rf)) {
                contour_count += 1;
            }
        }

        // Complex: many corners and contours that don't fit simple patterns
        if total_corners > 6 && contour_count > 5 {
            ((total_corners + contour_count) as f32 * 0.5).min(20.0)
        } else {
            0.0
        }
    }

    /// Check if position is within receptive field
    fn in_receptive_field(&self, x: usize, y: usize, rf: usize) -> bool {
        x >= self.x.saturating_sub(rf) 
            && x <= self.x + rf 
            && y >= self.y.saturating_sub(rf) 
            && y <= self.y + rf
    }

    /// Returns current activation level
    pub fn activation(&self) -> f32 {
        self.activation
    }

    /// Returns position of this detector
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns shape type this detector is tuned for
    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }
}

/// V4 cortex - processes complex shapes
#[derive(Debug)]
pub struct V4Cortex {
    shape_detectors: Vec<V4ShapeDetector>,
    width: usize,
    height: usize,
}

impl V4Cortex {
    /// Creates a new V4 cortex
    /// 
    /// # Arguments
    /// * `width`, `height` - Dimensions of visual field
    /// * `spacing` - Distance between detector centers
    pub fn new(width: usize, height: usize, spacing: usize) -> Self {
        let mut shape_detectors = Vec::new();
        let mut id = 0;

        let shape_types = vec![
            ShapeType::Circle,
            ShapeType::Rectangle,
            ShapeType::Triangle,
            ShapeType::Line,
            ShapeType::Cross,
            ShapeType::Complex,
        ];

        // Create shape detectors at regular intervals
        for y in (spacing..height - spacing).step_by(spacing) {
            for x in (spacing..width - spacing).step_by(spacing) {
                for &shape_type in &shape_types {
                    shape_detectors.push(V4ShapeDetector::new(
                        id,
                        x,
                        y,
                        shape_type,
                        10, // Larger receptive field than V2
                    ));
                    id += 1;
                }
            }
        }

        Self {
            shape_detectors,
            width,
            height,
        }
    }

    /// Process V2 output through V4
    pub fn process(&mut self, v2_response: &V2Response) -> V4Response {
        // Detect shapes
        for detector in &mut self.shape_detectors {
            detector.compute_response(v2_response);
        }

        // Count activations by shape type
        let mut type_activations = std::collections::HashMap::new();
        for detector in &self.shape_detectors {
            if detector.activation() > 5.0 {
                *type_activations.entry(detector.shape_type()).or_insert(0) += 1;
            }
        }

        // Create shape map - keep strongest detector at each position
        let mut shape_map = vec![vec![None; self.width]; self.height];
        let mut shape_count = 0;
        let mut activation_map = vec![vec![0.0; self.width]; self.height];

        for detector in &self.shape_detectors {
            if detector.activation() > 5.0 {  // Threshold for shape detection
                let (x, y) = detector.position();
                if x < self.width && y < self.height {
                    // Keep the strongest detector at this position
                    if detector.activation() > activation_map[y][x] {
                        activation_map[y][x] = detector.activation();
                        shape_map[y][x] = Some(detector.shape_type());
                    }
                    shape_count += 1;
                }
            }
        }

        V4Response {
            shape_map,
            shape_count,
            shape_type_counts: type_activations,
        }
    }

    /// Returns all shape detectors
    pub fn shape_detectors(&self) -> &[V4ShapeDetector] {
        &self.shape_detectors
    }
}

/// Response from V4 processing
#[derive(Debug)]
pub struct V4Response {
    /// Map of detected shapes (strongest at each position)
    pub shape_map: Vec<Vec<Option<ShapeType>>>,
    
    /// Number of shape detections (total activations)
    pub shape_count: usize,
    
    /// Count of each shape type detected
    pub shape_type_counts: std::collections::HashMap<ShapeType, usize>,
}

impl V4Response {
    /// Get dominant shape type
    pub fn dominant_shape_type(&self) -> Option<ShapeType> {
        self.shape_type_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(shape_type, _)| *shape_type)
    }

    /// Count shapes by type
    pub fn count_by_type(&self) -> &std::collections::HashMap<ShapeType, usize> {
        &self.shape_type_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4_cortex_creation() {
        let v4 = V4Cortex::new(64, 64, 8);
        assert!(v4.shape_detectors.len() > 0);
    }

    #[test]
    fn test_shape_detector_creation() {
        let detector = V4ShapeDetector::new(0, 10, 10, ShapeType::Circle, 5);
        assert_eq!(detector.position(), (10, 10));
        assert_eq!(detector.shape_type(), ShapeType::Circle);
    }
}
