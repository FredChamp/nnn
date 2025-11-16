//! V1 Cortex - Primary visual cortex with orientation-selective neurons

use std::f32::consts::PI;

/// Orientation preference of a V1 neuron (in degrees)
#[derive(Debug, Clone, Copy)]
pub struct Orientation(f32);

impl Orientation {
    /// Creates a new orientation
    pub fn new(degrees: f32) -> Self {
        Self(degrees % 180.0)
    }

    /// Returns the orientation in degrees
    pub fn degrees(&self) -> f32 {
        self.0
    }

    /// Returns the orientation in radians
    pub fn radians(&self) -> f32 {
        self.0 * PI / 180.0
    }

    /// Standard orientations
    pub fn horizontal() -> Self {
        Self(0.0)
    }

    pub fn vertical() -> Self {
        Self(90.0)
    }

    pub fn diagonal_right() -> Self {
        Self(45.0)
    }

    pub fn diagonal_left() -> Self {
        Self(135.0)
    }
}

/// Type of V1 neuron
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V1NeuronType {
    /// Simple cells: Respond to specific position and orientation
    Simple,
    /// Complex cells: Respond to orientation regardless of exact position
    Complex,
}

/// V1 neuron with orientation selectivity (inspired by Hubel & Wiesel)
#[derive(Debug)]
pub struct V1Neuron {
    id: usize,
    neuron_type: V1NeuronType,
    
    // Receptive field properties
    x: usize,
    y: usize,
    preferred_orientation: Orientation,
    receptive_field_size: usize,
    
    // Response
    activation: f32,
}

impl V1Neuron {
    /// Creates a new V1 neuron
    pub fn new(
        id: usize,
        neuron_type: V1NeuronType,
        x: usize,
        y: usize,
        preferred_orientation: Orientation,
        receptive_field_size: usize,
    ) -> Self {
        Self {
            id,
            neuron_type,
            x,
            y,
            preferred_orientation,
            receptive_field_size,
            activation: 0.0,
        }
    }

    /// Returns the neuron ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the neuron type
    pub fn neuron_type(&self) -> V1NeuronType {
        self.neuron_type
    }

    /// Returns the preferred orientation
    pub fn preferred_orientation(&self) -> Orientation {
        self.preferred_orientation
    }

    /// Returns the current activation
    pub fn activation(&self) -> f32 {
        self.activation
    }

    /// Computes the Gabor-like filter response
    ///
    /// Simple cells respond to oriented edges at specific positions
    /// Complex cells pool over positions but maintain orientation selectivity
    pub fn compute_response(&mut self, edge_map: &[Vec<f32>]) {
        if edge_map.is_empty() {
            return;
        }

        let height = edge_map.len();
        let width = edge_map[0].len();

        let mut response = 0.0;
        let mut count = 0;

        let angle = self.preferred_orientation.radians();
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        // Sample the receptive field
        let rf_radius = self.receptive_field_size as i32;
        
        for dy in -rf_radius..=rf_radius {
            for dx in -rf_radius..=rf_radius {
                let px = self.x as i32 + dx;
                let py = self.y as i32 + dy;

                if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                    continue;
                }

                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                if distance > self.receptive_field_size as f32 {
                    continue;
                }

                // Gabor-like orientation filtering
                // Project position onto preferred orientation axis
                let projected = (dx as f32 * cos_angle + dy as f32 * sin_angle).abs();
                let perpendicular = (-dx as f32 * sin_angle + dy as f32 * cos_angle).abs();

                // Elongated receptive field along preferred orientation
                let orientation_weight = if perpendicular < 2.0 && projected < rf_radius as f32 {
                    (-perpendicular.powi(2) / 2.0).exp()
                } else {
                    0.0
                };

                let edge_strength = edge_map[py as usize][px as usize];
                response += edge_strength * orientation_weight;
                count += 1;
            }
        }

        self.activation = if count > 0 {
            (response / count as f32).max(0.0)
        } else {
            0.0
        };

        // Complex cells have broader tuning (less position-specific)
        if self.neuron_type == V1NeuronType::Complex {
            self.activation *= 1.2; // Slight boost for complex cells
        }
    }

    /// Returns whether this neuron is significantly activated
    pub fn is_active(&self, threshold: f32) -> bool {
        self.activation > threshold
    }
}

/// V1 cortical column - group of neurons with similar orientation preference
pub struct V1Column {
    neurons: Vec<V1Neuron>,
    orientation: Orientation,
}

impl V1Column {
    /// Creates a new V1 column
    pub fn new(
        start_id: usize,
        x: usize,
        y: usize,
        orientation: Orientation,
        rf_size: usize,
    ) -> Self {
        let mut neurons = Vec::new();

        // Each column has both simple and complex cells
        neurons.push(V1Neuron::new(
            start_id,
            V1NeuronType::Simple,
            x,
            y,
            orientation,
            rf_size,
        ));

        neurons.push(V1Neuron::new(
            start_id + 1,
            V1NeuronType::Complex,
            x,
            y,
            orientation,
            rf_size + 2, // Larger RF for complex cells
        ));

        Self {
            neurons,
            orientation,
        }
    }

    /// Process input through this column
    pub fn process(&mut self, edge_map: &[Vec<f32>]) {
        for neuron in &mut self.neurons {
            neuron.compute_response(edge_map);
        }
    }

    /// Returns maximum activation in the column
    pub fn max_activation(&self) -> f32 {
        self.neurons
            .iter()
            .map(|n| n.activation())
            .fold(0.0, f32::max)
    }

    /// Returns the preferred orientation of this column
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

/// V1 cortex layer with multiple orientation columns
pub struct V1Cortex {
    columns: Vec<V1Column>,
    width: usize,
    height: usize,
}

impl V1Cortex {
    /// Creates a new V1 cortex
    ///
    /// # Arguments
    /// * `width`, `height` - Dimensions of visual field
    /// * `spacing` - Distance between column centers
    /// * `rf_size` - Receptive field size
    pub fn new(width: usize, height: usize, spacing: usize, rf_size: usize) -> Self {
        let mut columns = Vec::new();
        let mut id = 0;

        // Standard orientations (every 45 degrees)
        let orientations = vec![
            Orientation::horizontal(),
            Orientation::diagonal_right(),
            Orientation::vertical(),
            Orientation::diagonal_left(),
        ];

        // Create columns at regular intervals
        for y in (rf_size..height - rf_size).step_by(spacing) {
            for x in (rf_size..width - rf_size).step_by(spacing) {
                // Create a column for each orientation at this location
                for &orientation in &orientations {
                    columns.push(V1Column::new(id, x, y, orientation, rf_size));
                    id += 10; // Space for neurons in column
                }
            }
        }

        Self {
            columns,
            width,
            height,
        }
    }

    /// Process the entire edge map through V1
    pub fn process_edges(&mut self, edge_map: &[Vec<f32>]) {
        for column in &mut self.columns {
            column.process(edge_map);
        }
    }

    /// Returns all columns
    pub fn columns(&self) -> &[V1Column] {
        &self.columns
    }

    /// Get dominant orientation at each location
    pub fn orientation_map(&self) -> Vec<Vec<Option<Orientation>>> {
        let mut map = vec![vec![None; self.width]; self.height];

        // Group columns by position
        let mut position_map: std::collections::HashMap<(usize, usize), Vec<&V1Column>> =
            std::collections::HashMap::new();

        for column in &self.columns {
            // Get position from first neuron
            if let Some(neuron) = column.neurons.first() {
                let pos = (neuron.x, neuron.y);
                position_map.entry(pos).or_insert_with(Vec::new).push(column);
            }
        }

        // Find dominant orientation at each position
        for ((x, y), columns) in position_map {
            if let Some(dominant) = columns
                .iter()
                .max_by(|a, b| a.max_activation().partial_cmp(&b.max_activation()).unwrap())
            {
                if dominant.max_activation() > 0.1 {
                    map[y][x] = Some(dominant.orientation());
                }
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_creation() {
        let h = Orientation::horizontal();
        assert_eq!(h.degrees(), 0.0);

        let v = Orientation::vertical();
        assert_eq!(v.degrees(), 90.0);
    }

    #[test]
    fn test_v1_neuron_creation() {
        let neuron = V1Neuron::new(
            0,
            V1NeuronType::Simple,
            10,
            10,
            Orientation::horizontal(),
            5,
        );

        assert_eq!(neuron.id(), 0);
        assert_eq!(neuron.neuron_type(), V1NeuronType::Simple);
    }

    #[test]
    fn test_horizontal_edge_detection() {
        let mut neuron = V1Neuron::new(
            0,
            V1NeuronType::Simple,
            10,
            10,
            Orientation::horizontal(),
            5,
        );

        // Create horizontal edge
        let mut edge_map = vec![vec![0.0; 20]; 20];
        for x in 0..20 {
            edge_map[10][x] = 1.0;
        }

        neuron.compute_response(&edge_map);
        assert!(neuron.activation() > 0.0);
    }

    #[test]
    fn test_v1_cortex_creation() {
        let cortex = V1Cortex::new(50, 50, 10, 5);
        assert!(!cortex.columns().is_empty());
    }
}
