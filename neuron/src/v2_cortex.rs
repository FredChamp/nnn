//! V2 Cortex - Secondary Visual Cortex
//! 
//! V2 processes the output from V1 and detects more complex features:
//! - Angles and corners (junctions of edges)
//! - Continuous contours (illusory contours)
//! - Texture patterns
//! - Stereo disparity (depth perception)

/// Types of corner junctions detected by V2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerType {
    /// L-junction: 90° angle (e.g., corner of a rectangle)
    LJunction,
    /// T-junction: Occlusion boundary (one edge ends at another)
    TJunction,
    /// X-junction: Two lines crossing
    XJunction,
    /// Y-junction: Three-way intersection
    YJunction,
}

/// V2 neuron that detects corners and junctions
#[derive(Debug)]
pub struct V2CornerDetector {
    id: usize,
    x: usize,
    y: usize,
    corner_type: CornerType,
    receptive_field_size: usize,
    activation: f32,
}

impl V2CornerDetector {
    /// Creates a new V2 corner detector
    pub fn new(
        id: usize,
        x: usize,
        y: usize,
        corner_type: CornerType,
        rf_size: usize,
    ) -> Self {
        Self {
            id,
            x,
            y,
            corner_type,
            receptive_field_size: rf_size,
            activation: 0.0,
        }
    }

    /// Compute response to V1 orientation map
    /// 
    /// Detects corners by finding specific combinations of orientations
    pub fn compute_response(&mut self, orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>]) {
        if orientation_map.is_empty() {
            return;
        }

        let height = orientation_map.len();
        let width = orientation_map[0].len();
        let rf = self.receptive_field_size as i32;

        let mut response = 0.0;
        let mut count = 0;

        match self.corner_type {
            CornerType::LJunction => {
                // Look for perpendicular edges (horizontal + vertical)
                response = self.detect_l_junction(orientation_map, width, height);
            }
            CornerType::TJunction => {
                // Look for one edge terminating at another
                response = self.detect_t_junction(orientation_map, width, height);
            }
            CornerType::XJunction => {
                // Look for crossing edges
                response = self.detect_x_junction(orientation_map, width, height);
            }
            CornerType::YJunction => {
                // Look for three-way intersection
                response = self.detect_y_junction(orientation_map, width, height);
            }
        }

        self.activation = response;
    }

    /// Detect L-junction (perpendicular edges)
    fn detect_l_junction(
        &self,
        orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>],
        width: usize,
        height: usize,
    ) -> f32 {
        let x = self.x;
        let y = self.y;

        if x >= width || y >= height {
            return 0.0;
        }

        let mut horizontal_count = 0;
        let mut vertical_count = 0;

        // Check neighborhood for horizontal and vertical edges
        let rf = self.receptive_field_size as i32;
        for dy in -rf..=rf {
            for dx in -rf..=rf {
                let px = x as i32 + dx;
                let py = y as i32 + dy;

                if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                    continue;
                }

                if let Some(orientation) = orientation_map[py as usize][px as usize] {
                    let deg = orientation.degrees();
                    
                    // Horizontal edges (0° or 180°)
                    if deg < 22.5 || deg > 157.5 {
                        horizontal_count += 1;
                    }
                    // Vertical edges (90°)
                    else if (67.5..=112.5).contains(&deg) {
                        vertical_count += 1;
                    }
                }
            }
        }

        // Debug: print first detector's results
        if horizontal_count >= 1 && vertical_count >= 1 {
            let strength = (horizontal_count.min(vertical_count) as f32) * 2.0;
            strength.min(100.0)
        } else {
            0.0
        }
    }

    /// Detect T-junction (occlusion)
    fn detect_t_junction(
        &self,
        orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>],
        width: usize,
        height: usize,
    ) -> f32 {
        // T-junctions occur at occlusion boundaries
        // Similar to L-junction but with asymmetry
        let l_response = self.detect_l_junction(orientation_map, width, height);
        
        // T-junctions are typically slightly weaker than L-junctions
        l_response * 0.8
    }

    /// Detect X-junction (crossing lines)
    fn detect_x_junction(
        &self,
        orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>],
        width: usize,
        height: usize,
    ) -> f32 {
        let x = self.x;
        let y = self.y;

        if x >= width || y >= height {
            return 0.0;
        }

        let mut diagonal_45_count = 0;
        let mut diagonal_135_count = 0;

        let rf = self.receptive_field_size as i32;
        for dy in -rf..=rf {
            for dx in -rf..=rf {
                let px = x as i32 + dx;
                let py = y as i32 + dy;

                if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                    continue;
                }

                if let Some(orientation) = orientation_map[py as usize][px as usize] {
                    let deg = orientation.degrees();
                    
                    // Diagonal 45°
                    if (22.5..=67.5).contains(&deg) {
                        diagonal_45_count += 1;
                    }
                    // Diagonal 135°
                    else if (112.5..=157.5).contains(&deg) {
                        diagonal_135_count += 1;
                    }
                }
            }
        }

        // X-junction requires both diagonal orientations
        if diagonal_45_count >= 1 && diagonal_135_count >= 1 {
            let strength = (diagonal_45_count.min(diagonal_135_count) as f32) * 2.0;
            strength.min(100.0)
        } else {
            0.0
        }
    }

    /// Detect Y-junction (three-way intersection)
    fn detect_y_junction(
        &self,
        orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>],
        width: usize,
        height: usize,
    ) -> f32 {
        // Y-junctions are less common, combine multiple orientations
        let l_strength = self.detect_l_junction(orientation_map, width, height);
        let x_strength = self.detect_x_junction(orientation_map, width, height);
        
        // Y-junction is a complex combination
        ((l_strength + x_strength) / 2.0) * 0.7
    }

    /// Returns current activation
    pub fn activation(&self) -> f32 {
        self.activation
    }

    /// Returns position
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns corner type
    pub fn corner_type(&self) -> CornerType {
        self.corner_type
    }
}

/// V2 contour detector - follows continuous edges
#[derive(Debug)]
pub struct V2ContourDetector {
    id: usize,
    path_length: usize,
    curvature_threshold: f32,
    activation: f32,
}

impl V2ContourDetector {
    /// Creates a new contour detector
    pub fn new(id: usize, path_length: usize, curvature_threshold: f32) -> Self {
        Self {
            id,
            path_length,
            curvature_threshold,
            activation: 0.0,
        }
    }

    /// Detect contours (continuous edge paths)
    pub fn detect_contours(&mut self, edge_map: &[Vec<f32>]) -> Vec<Vec<(usize, usize)>> {
        if edge_map.is_empty() {
            return Vec::new();
        }

        let height = edge_map.len();
        let width = edge_map[0].len();
        
        // Dilate edge map to connect nearby edges
        let dilated_map = self.dilate_edge_map(edge_map);
        
        let mut visited = vec![vec![false; width]; height];
        let mut contours = Vec::new();

        // Find contour starting points (strong edges)
        for y in 0..height {
            for x in 0..width {
                if dilated_map[y][x] > 0.5 && !visited[y][x] {
                    if let Some(contour) = self.trace_contour(&dilated_map, &mut visited, x, y) {
                        if contour.len() >= self.path_length {
                            contours.push(contour);
                        }
                    }
                }
            }
        }

        self.activation = contours.len() as f32 * 10.0;
        contours
    }

    /// Trace a contour from a starting point
    fn trace_contour(
        &self,
        edge_map: &[Vec<f32>],
        visited: &mut [Vec<bool>],
        start_x: usize,
        start_y: usize,
    ) -> Option<Vec<(usize, usize)>> {
        let height = edge_map.len();
        let width = edge_map[0].len();
        let mut contour = vec![(start_x, start_y)];
        visited[start_y][start_x] = true;

        let mut current_x = start_x;
        let mut current_y = start_y;

        // Follow the contour by finding neighboring edge pixels
        for _ in 0..200 {
            // Max path length
            let mut best_neighbor = None;
            let mut best_strength = 0.0;

            // Check 8-connected neighbors
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = current_x as i32 + dx;
                    let ny = current_y as i32 + dy;

                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    if !visited[ny][nx] && edge_map[ny][nx] > 0.01 {  // Very low threshold to connect sparse edges
                        if edge_map[ny][nx] > best_strength {
                            best_strength = edge_map[ny][nx];
                            best_neighbor = Some((nx, ny));
                        }
                    }
                }
            }

            if let Some((nx, ny)) = best_neighbor {
                contour.push((nx, ny));
                visited[ny][nx] = true;
                current_x = nx;
                current_y = ny;
            } else {
                break; // No more neighbors, end of contour
            }
        }

        // Return contour if it has at least 1 point
        if contour.is_empty() {
            None
        } else {
            Some(contour)
        }
    }

    /// Returns current activation
    pub fn activation(&self) -> f32 {
        self.activation
    }

    /// Dilate edge map to connect nearby edges
    fn dilate_edge_map(&self, edge_map: &[Vec<f32>]) -> Vec<Vec<f32>> {
        let height = edge_map.len();
        let width = edge_map[0].len();
        let mut dilated = vec![vec![0.0; width]; height];

        for y in 0..height {
            for x in 0..width {
                let mut max_val = edge_map[y][x];
                
                // Check 3x3 neighborhood
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        
                        if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                            max_val = max_val.max(edge_map[ny as usize][nx as usize]);
                        }
                    }
                }
                
                dilated[y][x] = max_val;
            }
        }
        
        dilated
    }
}

/// V2 Cortex layer combining multiple feature detectors
pub struct V2Cortex {
    corner_detectors: Vec<V2CornerDetector>,
    contour_detector: V2ContourDetector,
    width: usize,
    height: usize,
}

impl V2Cortex {
    /// Creates a new V2 cortex
    /// 
    /// # Arguments
    /// * `width`, `height` - Dimensions of visual field
    /// * `spacing` - Distance between detector centers
    pub fn new(width: usize, height: usize, spacing: usize) -> Self {
        let mut corner_detectors = Vec::new();
        let mut id = 0;

        let corner_types = vec![
            CornerType::LJunction,
            CornerType::TJunction,
            CornerType::XJunction,
            CornerType::YJunction,
        ];

        // Create corner detectors at regular intervals
        for y in (spacing..height - spacing).step_by(spacing) {
            for x in (spacing..width - spacing).step_by(spacing) {
                for &corner_type in &corner_types {
                    corner_detectors.push(V2CornerDetector::new(
                        id,
                        x,
                        y,
                        corner_type,
                        6, // Receptive field size (increased from 3 to 6)
                    ));
                    id += 1;
                }
            }
        }

        let contour_detector = V2ContourDetector::new(0, 3, 0.5); // Reduced from 5 to 3

        Self {
            corner_detectors,
            contour_detector,
            width,
            height,
        }
    }

    /// Process V1 output through V2
    pub fn process(
        &mut self,
        orientation_map: &[Vec<Option<crate::v1_cortex::Orientation>>],
        edge_map: &[Vec<f32>],
    ) -> V2Response {
        // Detect corners and junctions
        for detector in &mut self.corner_detectors {
            detector.compute_response(orientation_map);
        }

        // Detect contours
        let contours = self.contour_detector.detect_contours(edge_map);
        let contour_count = contours.len();

        // Create corner map
        let mut corner_map = vec![vec![None; self.width]; self.height];
        for detector in &self.corner_detectors {
            if detector.activation() > 1.0 {  // Lowered threshold from 10.0 to 1.0
                let (x, y) = detector.position();
                corner_map[y][x] = Some(detector.corner_type());
            }
        }

        V2Response {
            corner_map,
            contours,
            corner_count: self.corner_detectors.iter().filter(|d| d.activation() > 1.0).count(),
            contour_count,
        }
    }

    /// Returns all corner detectors
    pub fn corner_detectors(&self) -> &[V2CornerDetector] {
        &self.corner_detectors
    }
}

/// Response from V2 processing
#[derive(Debug)]
pub struct V2Response {
    /// Map of detected corners
    pub corner_map: Vec<Vec<Option<CornerType>>>,
    
    /// List of detected contours (paths of connected edge pixels)
    pub contours: Vec<Vec<(usize, usize)>>,
    
    /// Number of corners detected
    pub corner_count: usize,
    
    /// Number of contours detected
    pub contour_count: usize,
}

impl V2Response {
    /// Get total feature count
    pub fn total_features(&self) -> usize {
        self.corner_count + self.contour_count
    }

    /// Get dominant corner type
    pub fn dominant_corner_type(&self) -> Option<CornerType> {
        let mut l_count = 0;
        let mut t_count = 0;
        let mut x_count = 0;
        let mut y_count = 0;

        for row in &self.corner_map {
            for corner_opt in row {
                if let Some(corner) = corner_opt {
                    match corner {
                        CornerType::LJunction => l_count += 1,
                        CornerType::TJunction => t_count += 1,
                        CornerType::XJunction => x_count += 1,
                        CornerType::YJunction => y_count += 1,
                    }
                }
            }
        }

        let max = l_count.max(t_count).max(x_count).max(y_count);
        
        if max == 0 {
            None
        } else if max == l_count {
            Some(CornerType::LJunction)
        } else if max == t_count {
            Some(CornerType::TJunction)
        } else if max == x_count {
            Some(CornerType::XJunction)
        } else {
            Some(CornerType::YJunction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_creation() {
        let v2 = V2Cortex::new(64, 64, 8);
        assert!(v2.corner_detectors.len() > 0);
    }

    #[test]
    fn test_corner_detector_creation() {
        let detector = V2CornerDetector::new(0, 10, 10, CornerType::LJunction, 3);
        assert_eq!(detector.position(), (10, 10));
        assert_eq!(detector.corner_type(), CornerType::LJunction);
    }

    #[test]
    fn test_contour_detection() {
        // Create a simple edge map with a horizontal line
        let mut edge_map = vec![vec![0.0; 20]; 20];
        for x in 5..15 {
            edge_map[10][x] = 5.0;
        }

        let mut detector = V2ContourDetector::new(0, 3, 0.5);
        let contours = detector.detect_contours(&edge_map);
        
        assert!(contours.len() > 0);
        assert!(detector.activation() > 0.0);
    }
}
