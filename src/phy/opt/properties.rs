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
    /// Asymmetry parameter.
    asym_param: f64,
    /// Refractive index.
    ref_index: f64,
}

impl Properties {
    /// Construct a new instance.
    #[pre(0.0 < ref_index)]
    #[pre(0.0 < scat_coeff)]
    #[pre(0.0 < abs_coeff)]
    #[pre(-1.0 < asym_param)]
    #[pre(asym_param < 1.0)]
    pub fn new(ref_index: f64, scat_coeff: f64, abs_coeff: f64, asym_param: f64) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            asym_param,
        }
    }

    /// Get the refractive index.
    pub fn ref_index(&self) -> f64 {
        self.ref_index
    }

    /// Get the scattering coefficient.
    pub fn scat_coeff(&self) -> f64 {
        self.scat_coeff
    }

    /// Get the absorption coefficient.
    pub fn abs_coeff(&self) -> f64 {
        self.abs_coeff
    }

    /// Get the asymmetry parameter.
    pub fn asym_param(&self) -> f64 {
        self.asym_param
    }
}
