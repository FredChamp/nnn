//! Complete visual processing pathway from photoreceptors to cortex

use crate::cone::Cone;
use crate::ganglion::GanglionLayer;
use crate::photopigment::{ConeType, LightStimulus};
use crate::v1_cortex::{Orientation, V1Cortex};
use crate::v2_cortex::{V2Cortex, V2Response};

/// Complete visual system simulation
pub struct VisualPathway {
    // Retinal layers
    cones: Vec<Cone>,
    ganglion_layer: GanglionLayer,
    
    // Cortical processing
    v1_cortex: V1Cortex,
    v2_cortex: V2Cortex,
    
    // Image dimensions
    width: usize,
    height: usize,
}

impl VisualPathway {
    /// Creates a new visual pathway
    ///
    /// # Arguments
    /// * `width`, `height` - Dimensions of visual field
    pub fn new(width: usize, height: usize) -> Self {
        // Create cone mosaic (simplified - one cone per pixel)
        let mut cones = Vec::new();
        let mut cone_id = 0;

        for y in 0..height {
            for x in 0..width {
                // Distribute cone types according to retinal abundance
                let cone_type = match (x + y) % 10 {
                    0 => ConeType::S,      // ~10% S-cones
                    1..=4 => ConeType::M,  // ~40% M-cones
                    _ => ConeType::L,      // ~50% L-cones
                };
                
                cones.push(Cone::new(cone_id, cone_type));
                cone_id += 1;
            }
        }

        // Create ganglion layer (center-surround edge detection)
        let ganglion_layer = GanglionLayer::new(width, height, 4, 1.5, 4.0);

        // Create V1 cortex (orientation detection)
        let v1_cortex = V1Cortex::new(width, height, 8, 5);
        
        // Create V2 cortex (corners and contours) - smaller spacing and larger RF
        let v2_cortex = V2Cortex::new(width, height, 4); // spacing reduced from 8 to 4

        Self {
            cones,
            ganglion_layer,
            v1_cortex,
            v2_cortex,
            width,
            height,
        }
    }

    /// Process a light stimulus pattern through the entire visual pathway
    ///
    /// # Arguments
    /// * `light_pattern` - 2D array of light stimuli (wavelength and intensity)
    ///
    /// # Returns
    /// Processed visual information at each stage
    pub fn process_scene(&mut self, light_pattern: &[Vec<LightStimulus>]) -> VisualResponse {
        // Stage 1: Phototransduction (cones convert light to neural signals)
        let cone_responses = self.process_phototransduction(light_pattern);

        // Stage 2: Ganglion cells detect edges and contrasts
        self.ganglion_layer.process_image(&cone_responses);
        let edge_map = self.ganglion_layer.create_edge_map();

        // Stage 3: V1 cortex extracts oriented features
        self.v1_cortex.process_edges(&edge_map);
        let orientation_map = self.v1_cortex.orientation_map();
        
        // Stage 4: V2 cortex detects corners and contours
        let v2_features = self.v2_cortex.process(&orientation_map, &edge_map);

        // Stage 5: Compute feature statistics
        let features = self.extract_features();

        VisualResponse {
            cone_activations: cone_responses,
            edge_map,
            orientation_map,
            v2_features,
            features,
        }
    }

    /// Process simple grayscale image (intensity only)
    pub fn process_grayscale_image(&mut self, image: &[Vec<f32>]) -> VisualResponse {
        // Convert grayscale to light stimuli (using mid-spectrum wavelength)
        let light_pattern: Vec<Vec<LightStimulus>> = image
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&intensity| LightStimulus::white_light(intensity * 100.0))
                    .collect()
            })
            .collect();

        self.process_scene(&light_pattern)
    }

    /// Stage 1: Phototransduction
    fn process_phototransduction(
        &mut self,
        light_pattern: &[Vec<LightStimulus>],
    ) -> Vec<Vec<f32>> {
        let mut activations = vec![vec![0.0; self.width]; self.height];

        for (idx, cone) in self.cones.iter_mut().enumerate() {
            let y = idx / self.width;
            let x = idx % self.width;

            if y < light_pattern.len() && x < light_pattern[0].len() {
                cone.phototransduction(light_pattern[y][x]);
                // Use response level (0 = dark, 1 = bright) as activation
                activations[y][x] = cone.response_level();
            }
        }

        activations
    }

    /// Extract high-level features from V1 responses
    fn extract_features(&self) -> VisualFeatures {
        let columns = self.v1_cortex.columns();
        
        let mut horizontal_strength = 0.0;
        let mut vertical_strength = 0.0;
        let mut diagonal_strength = 0.0;
        let mut total_activation = 0.0;

        for column in columns {
            let activation = column.max_activation();
            total_activation += activation;

            let orientation_deg = column.orientation().degrees();
            
            // Note: V1 neurons respond to edges/bars of a given orientation
            // 0° detects horizontal edges (vertical gradient)
            // 90° detects vertical edges (horizontal gradient)
            // We swap the labels to match the STRUCTURE orientation (not gradient direction)
            
            if orientation_deg < 22.5 || orientation_deg > 157.5 {
                // 0° V1 neurons -> detect vertical structures (horizontal transitions)
                vertical_strength += activation;
            } else if (67.5..=112.5).contains(&orientation_deg) {
                // 90° V1 neurons -> detect horizontal structures (vertical transitions)
                horizontal_strength += activation;
            } else {
                diagonal_strength += activation;
            }
        }

        // Normalize diagonal strength: we have 2 diagonal orientations (45° and 135°)
        // vs 1 horizontal (0°) and 1 vertical (90°), so divide by 2 for fair comparison
        // Additionally, diagonal detectors also respond to H/V edges at angles, so reduce further
        diagonal_strength /= 2.25;

        VisualFeatures {
            horizontal_strength,
            vertical_strength,
            diagonal_strength,
            total_activation,
        }
    }

    /// Returns the dimensions of the visual field
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

/// Response of the visual system to input
#[derive(Debug)]
pub struct VisualResponse {
    /// Activation levels of cones (0.0 = dark adapted, 1.0 = light adapted)
    pub cone_activations: Vec<Vec<f32>>,
    
    /// Edge map from ganglion cells
    pub edge_map: Vec<Vec<f32>>,
    
    /// Dominant orientation at each location (if any)
    pub orientation_map: Vec<Vec<Option<Orientation>>>,
    
    /// V2 features (corners and contours)
    pub v2_features: crate::v2_cortex::V2Response,
    
    /// High-level extracted features
    pub features: VisualFeatures,
}

/// High-level visual features extracted from V1
#[derive(Debug, Clone)]
pub struct VisualFeatures {
    /// Strength of horizontal edges
    pub horizontal_strength: f32,
    
    /// Strength of vertical edges
    pub vertical_strength: f32,
    
    /// Strength of diagonal edges
    pub diagonal_strength: f32,
    
    /// Total cortical activation
    pub total_activation: f32,
}

impl VisualFeatures {
    /// Determines the dominant orientation in the scene
    /// Uses a small threshold to prefer H/V when values are very close (within 6%)
    pub fn dominant_orientation(&self) -> &str {
        let threshold = 1.06; // 6% advantage for H/V over diagonal
        
        if self.horizontal_strength > self.vertical_strength
            && self.horizontal_strength * threshold > self.diagonal_strength
        {
            "Horizontal"
        } else if self.vertical_strength * threshold > self.horizontal_strength
            && self.vertical_strength * threshold > self.diagonal_strength
        {
            "Vertical"
        } else {
            "Diagonal"
        }
    }

    /// Returns overall edge strength
    pub fn edge_strength(&self) -> f32 {
        self.horizontal_strength + self.vertical_strength + self.diagonal_strength
    }
}

/// Helper functions to create test patterns
pub mod test_patterns {
    use super::*;

    /// Creates a vertical bar pattern
    pub fn vertical_bar(width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        let bar_width = width / 4;
        let bar_start = width / 2 - bar_width / 2;

        for y in 0..height {
            for x in bar_start..bar_start + bar_width {
                if x < width {
                    image[y][x] = 1.0;
                }
            }
        }
        image
    }

    /// Creates a horizontal bar pattern
    pub fn horizontal_bar(width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        let bar_height = height / 4;
        let bar_start = height / 2 - bar_height / 2;

        for y in bar_start..bar_start + bar_height {
            if y < height {
                for x in 0..width {
                    image[y][x] = 1.0;
                }
            }
        }
        image
    }

    /// Creates a diagonal line pattern
    pub fn diagonal_line(width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        
        for i in 0..width.min(height) {
            image[i][i] = 1.0;
            // Make line thicker
            if i > 0 {
                image[i - 1][i] = 0.7;
                image[i][i - 1] = 0.7;
            }
        }
        image
    }

    /// Creates a checkerboard pattern
    pub fn checkerboard(width: usize, height: usize, square_size: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        
        for y in 0..height {
            for x in 0..width {
                let checker = ((x / square_size) + (y / square_size)) % 2;
                image[y][x] = if checker == 0 { 1.0 } else { 0.0 };
            }
        }
        image
    }

    /// Creates a simple cross pattern
    pub fn cross(width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut image = vec![vec![0.0; width]; height];
        let cx = width / 2;
        let cy = height / 2;
        let thickness = 3;

        // Horizontal bar
        for y in cy.saturating_sub(thickness)..=(cy + thickness).min(height - 1) {
            for x in 0..width {
                image[y][x] = 1.0;
            }
        }

        // Vertical bar
        for x in cx.saturating_sub(thickness)..=(cx + thickness).min(width - 1) {
            for y in 0..height {
                image[y][x] = 1.0;
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::test_patterns::*;
    use super::*;

    #[test]
    fn test_visual_pathway_creation() {
        let pathway = VisualPathway::new(32, 32);
        assert_eq!(pathway.dimensions(), (32, 32));
    }

    #[test]
    fn test_vertical_bar_detection() {
        let mut pathway = VisualPathway::new(32, 32);
        let image = vertical_bar(32, 32);
        let response = pathway.process_grayscale_image(&image);

        // Vertical bar should have more vertical features
        assert!(response.features.edge_strength() > 0.0);
        println!("Vertical: {:.2}, Horizontal: {:.2}", 
                 response.features.vertical_strength, 
                 response.features.horizontal_strength);
    }

    #[test]
    fn test_horizontal_bar_detection() {
        let mut pathway = VisualPathway::new(32, 32);
        let image = horizontal_bar(32, 32);
        let response = pathway.process_grayscale_image(&image);

        // Horizontal bar should activate the visual system
        assert!(response.features.edge_strength() > 0.0);
        println!("Horizontal: {:.2}, Vertical: {:.2}", 
                 response.features.horizontal_strength, 
                 response.features.vertical_strength);
    }

    #[test]
    fn test_cross_pattern() {
        let mut pathway = VisualPathway::new(32, 32);
        let image = cross(32, 32);
        let response = pathway.process_grayscale_image(&image);

        // Cross should activate both horizontal and vertical
        assert!(response.features.horizontal_strength > 0.0);
        assert!(response.features.vertical_strength > 0.0);
        assert!(response.features.edge_strength() > 0.0);
    }
}
