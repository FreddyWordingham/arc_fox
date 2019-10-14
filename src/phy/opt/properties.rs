//! Optical properties structure.

use contracts::pre;

/// Optical physical properties.
/// Contains parameters governing how photons interact within the material.
#[derive(Debug)]
pub struct Properties {
    /// Scattering coefficient [m^-1].
    scat_coeff: f64,
    /// Absorption coefficient [m^-1].
    abs_coeff: f64,
}

impl Properties {
    /// Construct a new instance.
    #[pre(0.0 < scat_coeff)]
    #[pre(0.0 < abs_coeff)]
    pub fn new(scat_coeff: f64, abs_coeff: f64) -> Self {
        Self {
            scat_coeff,
            abs_coeff,
        }
    }

    /// Get the scattering coefficient.
    pub fn scat_coeff(&self) -> f64 {
        self.scat_coeff
    }

    /// Get the absorption coefficient.
    pub fn abs_coeff(&self) -> f64 {
        self.abs_coeff
    }
}
