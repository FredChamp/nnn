//! Neurotransmitter types and their modulation effects

/// Types of neurotransmitters that can be released at synapses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Neurotransmitter {
    /// Excitatory neurotransmitter - increases likelihood of action potential
    Glutamate,
    /// Inhibitory neurotransmitter - decreases likelihood of action potential
    GABA,
    /// Modulatory neurotransmitter - reward and motivation
    Dopamine,
    /// Modulatory neurotransmitter - mood and cognition
    Serotonin,
}

impl Neurotransmitter {
    /// Returns the modulation factor for this neurotransmitter
    /// 
    /// # Returns
    /// - Positive values for excitatory effects
    /// - Negative values for inhibitory effects
    /// - Values < 1.0 for modulatory effects
    pub fn modulation_factor(&self) -> f32 {
        match self {
            Self::Glutamate => 1.0,      // Full excitatory effect
            Self::GABA => -0.5,          // Inhibitory effect
            Self::Dopamine => 0.8,       // Moderate excitatory modulation
            Self::Serotonin => 0.6,      // Mild excitatory modulation
        }
    }

    /// Returns whether this neurotransmitter is primarily excitatory
    pub fn is_excitatory(&self) -> bool {
        matches!(self, Self::Glutamate | Self::Dopamine | Self::Serotonin)
    }

    /// Returns whether this neurotransmitter is primarily inhibitory
    pub fn is_inhibitory(&self) -> bool {
        matches!(self, Self::GABA)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glutamate_is_excitatory() {
        assert!(Neurotransmitter::Glutamate.is_excitatory());
        assert!(!Neurotransmitter::Glutamate.is_inhibitory());
    }

    #[test]
    fn test_gaba_is_inhibitory() {
        assert!(Neurotransmitter::GABA.is_inhibitory());
        assert!(!Neurotransmitter::GABA.is_excitatory());
    }

    #[test]
    fn test_modulation_factors() {
        assert_eq!(Neurotransmitter::Glutamate.modulation_factor(), 1.0);
        assert_eq!(Neurotransmitter::GABA.modulation_factor(), -0.5);
        assert_eq!(Neurotransmitter::Dopamine.modulation_factor(), 0.8);
        assert_eq!(Neurotransmitter::Serotonin.modulation_factor(), 0.6);
    }
}
