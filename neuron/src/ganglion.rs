//! Ganglion cells - Edge and contrast detection through center-surround receptive fields

/// Type of ganglion cell response
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GanglionType {
    /// ON-center: Responds to light in center, inhibited by light in surround
    OnCenter,
    /// OFF-center: Responds to dark in center, inhibited by dark in surround
    OffCenter,
}

/// Ganglion cell with center-surround receptive field
#[derive(Debug)]
pub struct GanglionCell {
    id: usize,
    cell_type: GanglionType,
    
    // Position and receptive field
    x: usize,
    y: usize,
    center_radius: f32,
    surround_radius: f32,
    
    // Response state
    center_activation: f32,
    surround_activation: f32,
    output_rate: f32, // Firing rate in Hz
}

impl GanglionCell {
    /// Creates a new ganglion cell
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    /// * `cell_type` - ON-center or OFF-center
    /// * `x`, `y` - Position in visual field
    /// * `center_radius` - Radius of center region
    /// * `surround_radius` - Radius of surround region
    pub fn new(
        id: usize,
        cell_type: GanglionType,
        x: usize,
        y: usize,
        center_radius: f32,
        surround_radius: f32,
    ) -> Self {
        Self {
            id,
            cell_type,
            x,
            y,
            center_radius,
            surround_radius,
            center_activation: 0.0,
            surround_activation: 0.0,
            output_rate: 0.0,
        }
    }

    /// Returns the cell ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the cell type
    pub fn cell_type(&self) -> GanglionType {
        self.cell_type
    }

    /// Returns the position
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns the current firing rate
    pub fn firing_rate(&self) -> f32 {
        self.output_rate
    }

    /// Computes the response to an input image region
    ///
    /// # Arguments
    /// * `image` - 2D array of pixel intensities (0.0 to 1.0)
    ///
    /// The center-surround antagonism:
    /// - ON-center: Response = Center - Surround
    /// - OFF-center: Response = Surround - Center
    pub fn compute_response(&mut self, image: &[Vec<f32>]) {
        if image.is_empty() {
            return;
        }

        let height = image.len();
        let width = image[0].len();

        let mut center_sum = 0.0;
        let mut center_count = 0;
        let mut surround_sum = 0.0;
        let mut surround_count = 0;

        // Sample pixels in receptive field
        for dy in -(self.surround_radius as i32)..=(self.surround_radius as i32) {
            for dx in -(self.surround_radius as i32)..=(self.surround_radius as i32) {
                let px = self.x as i32 + dx;
                let py = self.y as i32 + dy;

                if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                    continue;
                }

                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                let intensity = image[py as usize][px as usize];

                if distance <= self.center_radius {
                    center_sum += intensity;
                    center_count += 1;
                } else if distance <= self.surround_radius {
                    surround_sum += intensity;
                    surround_count += 1;
                }
            }
        }

        self.center_activation = if center_count > 0 {
            center_sum / center_count as f32
        } else {
            0.0
        };

        self.surround_activation = if surround_count > 0 {
            surround_sum / surround_count as f32
        } else {
            0.0
        };

        // Compute center-surround difference
        let response = match self.cell_type {
            GanglionType::OnCenter => self.center_activation - self.surround_activation,
            GanglionType::OffCenter => self.surround_activation - self.center_activation,
        };

        // Convert to firing rate (rectified and scaled)
        self.output_rate = (response * 100.0).max(0.0);
    }

    /// Returns the center-surround difference (positive = active)
    pub fn response_strength(&self) -> f32 {
        match self.cell_type {
            GanglionType::OnCenter => self.center_activation - self.surround_activation,
            GanglionType::OffCenter => self.surround_activation - self.center_activation,
        }
    }
}

/// Layer of ganglion cells covering a visual field
pub struct GanglionLayer {
    cells: Vec<GanglionCell>,
    width: usize,
    height: usize,
}

impl GanglionLayer {
    /// Creates a new layer of ganglion cells
    ///
    /// # Arguments
    /// * `width`, `height` - Dimensions of visual field
    /// * `spacing` - Distance between cell centers
    /// * `center_radius` - Size of center region
    /// * `surround_radius` - Size of surround region
    pub fn new(
        width: usize,
        height: usize,
        spacing: usize,
        center_radius: f32,
        surround_radius: f32,
    ) -> Self {
        let mut cells = Vec::new();
        let mut id = 0;

        // Create a grid of ON and OFF center cells
        for y in (0..height).step_by(spacing) {
            for x in (0..width).step_by(spacing) {
                // Create both ON and OFF cells at each location
                cells.push(GanglionCell::new(
                    id,
                    GanglionType::OnCenter,
                    x,
                    y,
                    center_radius,
                    surround_radius,
                ));
                id += 1;

                cells.push(GanglionCell::new(
                    id,
                    GanglionType::OffCenter,
                    x,
                    y,
                    center_radius,
                    surround_radius,
                ));
                id += 1;
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    /// Processes an entire image through all ganglion cells
    pub fn process_image(&mut self, image: &[Vec<f32>]) {
        for cell in &mut self.cells {
            cell.compute_response(image);
        }
    }

    /// Returns all cells
    pub fn cells(&self) -> &[GanglionCell] {
        &self.cells
    }

    /// Returns cells of a specific type
    pub fn cells_by_type(&self, cell_type: GanglionType) -> Vec<&GanglionCell> {
        self.cells
            .iter()
            .filter(|c| c.cell_type() == cell_type)
            .collect()
    }

    /// Creates an edge map from ganglion responses
    pub fn create_edge_map(&self) -> Vec<Vec<f32>> {
        let mut edge_map = vec![vec![0.0; self.width]; self.height];

        for cell in &self.cells {
            let (x, y) = cell.position();
            if x < self.width && y < self.height {
                edge_map[y][x] += cell.response_strength().abs();
            }
        }

        edge_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: usize, height: usize) -> Vec<Vec<f32>> {
        vec![vec![0.5; width]; height]
    }

    fn create_edge_image(width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        // Create a vertical edge in the middle
        for y in 0..height {
            for x in 0..width {
                image[y][x] = if x < width / 2 { 0.0 } else { 1.0 };
            }
        }
        image
    }

    #[test]
    fn test_ganglion_creation() {
        let cell = GanglionCell::new(0, GanglionType::OnCenter, 10, 10, 2.0, 5.0);
        assert_eq!(cell.id(), 0);
        assert_eq!(cell.cell_type(), GanglionType::OnCenter);
        assert_eq!(cell.position(), (10, 10));
    }

    #[test]
    fn test_uniform_image_no_response() {
        let mut cell = GanglionCell::new(0, GanglionType::OnCenter, 5, 5, 2.0, 5.0);
        let image = create_test_image(10, 10);
        
        cell.compute_response(&image);
        
        // Uniform image should produce no response (center = surround)
        assert!(cell.response_strength().abs() < 0.01);
    }

    #[test]
    fn test_on_center_bright_spot() {
        let mut cell = GanglionCell::new(0, GanglionType::OnCenter, 5, 5, 2.0, 5.0);
        let mut image = vec![vec![0.0; 10]; 10];
        
        // Create bright spot in center
        image[5][5] = 1.0;
        
        cell.compute_response(&image);
        
        // ON-center should respond positively to bright center
        assert!(cell.response_strength() > 0.0);
    }

    #[test]
    fn test_off_center_dark_spot() {
        let mut cell = GanglionCell::new(0, GanglionType::OffCenter, 5, 5, 2.0, 5.0);
        let mut image = vec![vec![1.0; 10]; 10];
        
        // Create dark spot in center
        image[5][5] = 0.0;
        
        cell.compute_response(&image);
        
        // OFF-center should respond positively to dark center
        assert!(cell.response_strength() > 0.0);
    }

    #[test]
    fn test_ganglion_layer_creation() {
        let layer = GanglionLayer::new(20, 20, 5, 2.0, 5.0);
        assert!(!layer.cells().is_empty());
        
        let on_cells = layer.cells_by_type(GanglionType::OnCenter);
        let off_cells = layer.cells_by_type(GanglionType::OffCenter);
        
        assert_eq!(on_cells.len(), off_cells.len());
    }

    #[test]
    fn test_edge_detection() {
        let mut layer = GanglionLayer::new(20, 20, 3, 1.5, 4.0);
        let edge_image = create_edge_image(20, 20);
        
        layer.process_image(&edge_image);
        let edge_map = layer.create_edge_map();
        
        // Edge map should have some response
        let total_response: f32 = edge_map.iter().flatten().sum();
        assert!(total_response > 0.0);
    }
}
