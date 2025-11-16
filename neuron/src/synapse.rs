//! Synapse implementation - connection between neurons

use crate::neurotransmitter::Neurotransmitter;

/// A synapse represents a connection from one neuron to another
#[derive(Debug, Clone)]
pub struct Synapse {
    /// Synaptic weight - determines connection strength
    weight: f32,
    /// Type of neurotransmitter released at this synapse
    neurotransmitter: Neurotransmitter,
    /// ID of the target (postsynaptic) neuron
    target_id: usize,
}

impl Synapse {
    /// Creates a new synapse
    ///
    /// # Arguments
    /// * `target_id` - ID of the postsynaptic neuron
    /// * `weight` - Strength of the synaptic connection (typically 0.0 to 1.0)
    /// * `neurotransmitter` - Type of neurotransmitter used
    pub fn new(target_id: usize, weight: f32, neurotransmitter: Neurotransmitter) -> Self {
        Self {
            weight,
            neurotransmitter,
            target_id,
        }
    }

    /// Returns the target neuron ID
    pub fn target_id(&self) -> usize {
        self.target_id
    }

    /// Returns the synaptic weight
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Returns the neurotransmitter type
    pub fn neurotransmitter(&self) -> Neurotransmitter {
        self.neurotransmitter
    }

    /// Modulates a presynaptic signal based on synaptic properties
    ///
    /// # Arguments
    /// * `signal` - The presynaptic signal strength
    ///
    /// # Returns
    /// The modulated signal to be transmitted to the postsynaptic neuron
    pub fn modulate_signal(&self, signal: f32) -> f32 {
        signal * self.weight * self.neurotransmitter.modulation_factor()
    }

    /// Updates the synaptic weight (for plasticity mechanisms like LTP/LTD)
    pub fn update_weight(&mut self, delta: f32) {
        self.weight = (self.weight + delta).clamp(0.0, 2.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synapse_creation() {
        let synapse = Synapse::new(1, 0.8, Neurotransmitter::Glutamate);
        assert_eq!(synapse.target_id(), 1);
        assert_eq!(synapse.weight(), 0.8);
        assert_eq!(synapse.neurotransmitter(), Neurotransmitter::Glutamate);
    }

    #[test]
    fn test_excitatory_modulation() {
        let synapse = Synapse::new(1, 0.8, Neurotransmitter::Glutamate);
        let modulated = synapse.modulate_signal(10.0);
        assert_eq!(modulated, 8.0); // 10.0 * 0.8 * 1.0
    }

    #[test]
    fn test_inhibitory_modulation() {
        let synapse = Synapse::new(1, 0.5, Neurotransmitter::GABA);
        let modulated = synapse.modulate_signal(10.0);
        assert_eq!(modulated, -2.5); // 10.0 * 0.5 * -0.5
    }

    #[test]
    fn test_weight_update() {
        let mut synapse = Synapse::new(1, 0.5, Neurotransmitter::Glutamate);
        synapse.update_weight(0.3);
        assert_eq!(synapse.weight(), 0.8);
        
        // Test clamping
        synapse.update_weight(5.0);
        assert_eq!(synapse.weight(), 2.0);
    }
}
