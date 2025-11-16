//! Cone photoreceptor implementation with realistic phototransduction

use crate::photopigment::{ConeType, LightStimulus};

// Physiological constants for cones
const DARK_POTENTIAL: f32 = -40.0; // mV - depolarized in darkness
const LIGHT_POTENTIAL: f32 = -70.0; // mV - hyperpolarized in light
const DARK_GLUTAMATE_RELEASE: f32 = 100.0; // High glutamate in darkness
const LIGHT_GLUTAMATE_RELEASE: f32 = 10.0; // Low glutamate in light
const CGMP_DARK_LEVEL: f32 = 100.0; // High cGMP in darkness
const CGMP_LIGHT_LEVEL: f32 = 10.0; // Low cGMP in light

/// Represents a cone photoreceptor cell in the retina
#[derive(Debug)]
pub struct Cone {
    id: usize,
    cone_type: ConeType,
    
    // Anatomical segments
    outer_segment_pigment: f32, // Photopigment concentration
    inner_segment_atp: f32, // Energy reserves
    
    // Physiological state
    membrane_potential: f32,
    cgmp_level: f32, // Cyclic GMP concentration
    glutamate_release: f32,
    
    // Adaptation state
    adaptation_level: f32, // Light adaptation (0.0 = dark adapted, 1.0 = light adapted)
    
    // Connection to downstream neurons
    connected_neurons: Vec<usize>, // Bipolar cell IDs
}

impl Cone {
    /// Creates a new cone photoreceptor
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this cone
    /// * `cone_type` - Type of cone (S, M, or L)
    pub fn new(id: usize, cone_type: ConeType) -> Self {
        Self {
            id,
            cone_type,
            outer_segment_pigment: 100.0,
            inner_segment_atp: 100.0,
            membrane_potential: DARK_POTENTIAL,
            cgmp_level: CGMP_DARK_LEVEL,
            glutamate_release: DARK_GLUTAMATE_RELEASE,
            adaptation_level: 0.0,
            connected_neurons: Vec::new(),
        }
    }

    /// Returns the cone's ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the cone type
    pub fn cone_type(&self) -> ConeType {
        self.cone_type
    }

    /// Returns the current membrane potential
    pub fn membrane_potential(&self) -> f32 {
        self.membrane_potential
    }

    /// Returns the current glutamate release rate
    pub fn glutamate_release(&self) -> f32 {
        self.glutamate_release
    }

    /// Returns the cGMP level
    pub fn cgmp_level(&self) -> f32 {
        self.cgmp_level
    }

    /// Connects this cone to a bipolar neuron
    pub fn connect_to_neuron(&mut self, neuron_id: usize) {
        if !self.connected_neurons.contains(&neuron_id) {
            self.connected_neurons.push(neuron_id);
        }
    }

    /// Phototransduction cascade: converts light into electrical signal
    ///
    /// # Process:
    /// 1. Light activates photopigment (opsin)
    /// 2. Activated opsin triggers transducin (G-protein)
    /// 3. Transducin activates phosphodiesterase (PDE)
    /// 4. PDE hydrolyzes cGMP
    /// 5. Lower cGMP closes ion channels
    /// 6. Cell hyperpolarizes
    /// 7. Less glutamate released
    ///
    /// # Arguments
    /// * `light` - The light stimulus
    pub fn phototransduction(&mut self, light: LightStimulus) {
        // Calculate effective light intensity based on spectral sensitivity
        let sensitivity = self.cone_type.spectral_sensitivity(light.wavelength);
        let effective_intensity = light.intensity * sensitivity;
        
        // Apply adaptation: cones adapt to ambient light levels
        let adapted_intensity = effective_intensity * (1.0 - self.adaptation_level * 0.7);
        
        // Phototransduction cascade
        // More light → less cGMP
        let target_cgmp = CGMP_DARK_LEVEL - (adapted_intensity / 10.0).clamp(0.0, 90.0);
        
        // cGMP changes gradually (not instantaneous)
        let cgmp_change_rate = 0.3;
        self.cgmp_level += (target_cgmp - self.cgmp_level) * cgmp_change_rate;
        self.cgmp_level = self.cgmp_level.clamp(CGMP_LIGHT_LEVEL, CGMP_DARK_LEVEL);
        
        // cGMP-gated channels: more cGMP → more open channels → more depolarized
        let channel_opening = self.cgmp_level / CGMP_DARK_LEVEL;
        self.membrane_potential = LIGHT_POTENTIAL + (DARK_POTENTIAL - LIGHT_POTENTIAL) * channel_opening;
        
        // Glutamate release is proportional to depolarization
        let depolarization_factor = (self.membrane_potential - LIGHT_POTENTIAL) / (DARK_POTENTIAL - LIGHT_POTENTIAL);
        self.glutamate_release = LIGHT_GLUTAMATE_RELEASE 
            + (DARK_GLUTAMATE_RELEASE - LIGHT_GLUTAMATE_RELEASE) * depolarization_factor;
        
        // Light adaptation: gradually adapt to sustained light
        let adaptation_rate = 0.01;
        let target_adaptation = (effective_intensity / 100.0).clamp(0.0, 1.0);
        self.adaptation_level += (target_adaptation - self.adaptation_level) * adaptation_rate;
        
        // Energy consumption (ATP usage)
        self.inner_segment_atp = (self.inner_segment_atp - 0.1).max(20.0);
    }

    /// Regenerates photopigment and ATP (recovery in darkness)
    pub fn metabolic_recovery(&mut self) {
        // Regenerate photopigment
        self.outer_segment_pigment = (self.outer_segment_pigment + 0.5).min(100.0);
        
        // Regenerate ATP
        self.inner_segment_atp = (self.inner_segment_atp + 1.0).min(100.0);
    }

    /// Returns signals to be transmitted to connected neurons
    ///
    /// # Returns
    /// Vector of (neuron_id, signal_strength) tuples
    /// Note: Signal is inhibitory when glutamate is high (darkness)
    pub fn transmit_to_neurons(&self) -> Vec<(usize, f32)> {
        self.connected_neurons
            .iter()
            .map(|&neuron_id| {
                // Convert glutamate to signal
                // High glutamate (dark) = inhibitory signal
                // Low glutamate (light) = less inhibition (excitation)
                let signal = -self.glutamate_release / 10.0 + 10.0; // Inverted signal
                (neuron_id, signal)
            })
            .collect()
    }

    /// Returns the adaptation level
    pub fn adaptation_level(&self) -> f32 {
        self.adaptation_level
    }

    /// Returns whether the cone is in a light-adapted state
    pub fn is_light_adapted(&self) -> bool {
        self.adaptation_level > 0.5
    }

    /// Returns the cone's response level (0.0 = dark, 1.0 = bright light)
    /// Amplified for better visualization
    pub fn response_level(&self) -> f32 {
        let base_response = (DARK_POTENTIAL - self.membrane_potential) / (DARK_POTENTIAL - LIGHT_POTENTIAL);
        // Amplify the response by 50x for better detection
        (base_response * 50.0).min(1.0)
    }

    /// Returns energy status (ATP percentage)
    pub fn energy_level(&self) -> f32 {
        self.inner_segment_atp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cone_creation() {
        let cone = Cone::new(0, ConeType::M);
        assert_eq!(cone.id(), 0);
        assert_eq!(cone.cone_type(), ConeType::M);
        assert_eq!(cone.membrane_potential(), DARK_POTENTIAL);
        assert_eq!(cone.cgmp_level(), CGMP_DARK_LEVEL);
    }

    #[test]
    fn test_phototransduction_darkness() {
        let mut cone = Cone::new(0, ConeType::L);
        cone.phototransduction(LightStimulus::darkness());
        
        // Should remain near dark potential
        assert!(cone.membrane_potential() > -45.0);
        assert!(cone.glutamate_release() > 80.0);
    }

    #[test]
    fn test_phototransduction_light() {
        let mut cone = Cone::new(0, ConeType::L);
        
        // Apply bright red light (L-cone's peak)
        for _ in 0..50 {
            cone.phototransduction(LightStimulus::red(100.0));
        }
        
        // Should hyperpolarize (move towards light potential)
        assert!(cone.membrane_potential() < DARK_POTENTIAL);
        assert!(cone.glutamate_release() < DARK_GLUTAMATE_RELEASE);
        assert!(cone.cgmp_level() < CGMP_DARK_LEVEL);
    }

    #[test]
    fn test_spectral_selectivity() {
        let mut s_cone = Cone::new(0, ConeType::S);
        let mut l_cone = Cone::new(1, ConeType::L);
        
        // Blue light should activate S-cone more than L-cone
        for _ in 0..10 {
            s_cone.phototransduction(LightStimulus::blue(100.0));
            l_cone.phototransduction(LightStimulus::blue(100.0));
        }
        
        assert!(s_cone.response_level() > l_cone.response_level());
    }

    #[test]
    fn test_light_adaptation() {
        let mut cone = Cone::new(0, ConeType::M);
        
        // Sustained light should cause adaptation
        for _ in 0..100 {
            cone.phototransduction(LightStimulus::green(100.0));
        }
        
        assert!(cone.is_light_adapted());
        assert!(cone.adaptation_level() > 0.5);
    }

    #[test]
    fn test_neuron_connection() {
        let mut cone = Cone::new(0, ConeType::S);
        cone.connect_to_neuron(10);
        cone.connect_to_neuron(11);
        
        let transmissions = cone.transmit_to_neurons();
        assert_eq!(transmissions.len(), 2);
    }

    #[test]
    fn test_metabolic_recovery() {
        let mut cone = Cone::new(0, ConeType::M);
        
        // Deplete energy
        for _ in 0..100 {
            cone.phototransduction(LightStimulus::green(100.0));
        }
        
        let depleted_atp = cone.energy_level();
        
        // Recover
        for _ in 0..50 {
            cone.metabolic_recovery();
        }
        
        assert!(cone.energy_level() > depleted_atp);
    }
}
