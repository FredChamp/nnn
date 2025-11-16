//! Photopigment types and spectral sensitivity

/// Types of cone photopigments with different spectral sensitivities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConeType {
    /// S-cones: Short wavelength sensitive (~420 nm, blue)
    S,
    /// M-cones: Medium wavelength sensitive (~530 nm, green)
    M,
    /// L-cones: Long wavelength sensitive (~560 nm, red)
    L,
}

impl ConeType {
    /// Returns the peak wavelength sensitivity in nanometers
    pub fn peak_wavelength(&self) -> f32 {
        match self {
            Self::S => 420.0,
            Self::M => 530.0,
            Self::L => 560.0,
        }
    }

    /// Returns the standard deviation of the sensitivity curve
    fn sensitivity_width(&self) -> f32 {
        match self {
            Self::S => 30.0,
            Self::M => 40.0,
            Self::L => 40.0,
        }
    }

    /// Calculates the spectral sensitivity for a given wavelength
    /// Uses a Gaussian approximation of cone spectral sensitivity
    ///
    /// # Arguments
    /// * `wavelength` - Light wavelength in nanometers (380-780 nm)
    ///
    /// # Returns
    /// Normalized sensitivity value between 0.0 and 1.0
    pub fn spectral_sensitivity(&self, wavelength: f32) -> f32 {
        let peak = self.peak_wavelength();
        let width = self.sensitivity_width();
        
        // Gaussian distribution
        let exponent = -((wavelength - peak).powi(2)) / (2.0 * width.powi(2));
        (exponent.exp()).clamp(0.0, 1.0)
    }

    /// Returns the name of the cone type
    pub fn name(&self) -> &'static str {
        match self {
            Self::S => "S-cone (Blue)",
            Self::M => "M-cone (Green)",
            Self::L => "L-cone (Red)",
        }
    }

    /// Returns the relative abundance in the retina (as percentage)
    pub fn abundance(&self) -> f32 {
        match self {
            Self::S => 5.0,   // ~5% of cones
            Self::M => 33.0,  // ~33% of cones
            Self::L => 62.0,  // ~62% of cones
        }
    }
}

/// Represents a light stimulus with wavelength and intensity
#[derive(Debug, Clone, Copy)]
pub struct LightStimulus {
    /// Wavelength in nanometers (380-780 nm for visible light)
    pub wavelength: f32,
    /// Intensity in photons per second (arbitrary units)
    pub intensity: f32,
}

impl LightStimulus {
    /// Creates a new light stimulus
    pub fn new(wavelength: f32, intensity: f32) -> Self {
        Self {
            wavelength: wavelength.clamp(380.0, 780.0),
            intensity: intensity.max(0.0),
        }
    }

    /// Creates darkness (no light)
    pub fn darkness() -> Self {
        Self {
            wavelength: 555.0, // Peak human sensitivity
            intensity: 0.0,
        }
    }

    /// Creates white light (combination of all wavelengths)
    pub fn white_light(intensity: f32) -> Self {
        Self::new(555.0, intensity) // Use peak sensitivity wavelength
    }

    /// Creates colored light presets
    pub fn blue(intensity: f32) -> Self {
        Self::new(470.0, intensity)
    }

    pub fn green(intensity: f32) -> Self {
        Self::new(530.0, intensity)
    }

    pub fn red(intensity: f32) -> Self {
        Self::new(650.0, intensity)
    }

    pub fn yellow(intensity: f32) -> Self {
        Self::new(580.0, intensity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peak_wavelengths() {
        assert_eq!(ConeType::S.peak_wavelength(), 420.0);
        assert_eq!(ConeType::M.peak_wavelength(), 530.0);
        assert_eq!(ConeType::L.peak_wavelength(), 560.0);
    }

    #[test]
    fn test_spectral_sensitivity_at_peak() {
        // Sensitivity should be maximum (1.0) at peak wavelength
        assert!((ConeType::S.spectral_sensitivity(420.0) - 1.0).abs() < 0.01);
        assert!((ConeType::M.spectral_sensitivity(530.0) - 1.0).abs() < 0.01);
        assert!((ConeType::L.spectral_sensitivity(560.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_spectral_sensitivity_decreases_away_from_peak() {
        let s_at_peak = ConeType::S.spectral_sensitivity(420.0);
        let s_off_peak = ConeType::S.spectral_sensitivity(500.0);
        assert!(s_at_peak > s_off_peak);
    }

    #[test]
    fn test_light_stimulus_clamping() {
        let light = LightStimulus::new(1000.0, -5.0);
        assert_eq!(light.wavelength, 780.0); // Clamped to max
        assert_eq!(light.intensity, 0.0); // Clamped to min
    }

    #[test]
    fn test_color_presets() {
        let blue = LightStimulus::blue(100.0);
        let green = LightStimulus::green(100.0);
        let red = LightStimulus::red(100.0);
        
        assert!(blue.wavelength < 500.0);
        assert!(green.wavelength > 500.0 && green.wavelength < 600.0);
        assert!(red.wavelength > 600.0);
    }

    #[test]
    fn test_cone_abundance() {
        let total = ConeType::S.abundance() 
                  + ConeType::M.abundance() 
                  + ConeType::L.abundance();
        assert!((total - 100.0).abs() < 0.1);
    }
}
