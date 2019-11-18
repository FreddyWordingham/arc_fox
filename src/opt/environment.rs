//! Environment structure.

use contracts::{post, pre};

/// Environment structure implementation.
/// Information about the local optical environment.
#[derive(Debug)]
pub struct Environment {
    /// Refractive index.
    ref_index: f64,
    /// Interaction coefficient. [m^-1]
    inter_coeff: f64,
    /// Single scattering albedo.
    albedo: f64,
    /// Shift probability.
    shift_prob: f64,
    /// Asymmetry parameter.
    asym: f64,
}

impl Environment {
    /// Construct a new instance.
    #[pre(ref_index > 0.0)]
    #[pre(scat_coeff > 0.0)]
    #[pre(abs_coeff >= 0.0)]
    #[pre(shift_coeff >= 0.0)]
    #[pre(asym > -1.0)]
    #[pre(asym < 1.0)]
    pub fn new(
        ref_index: f64,
        scat_coeff: f64,
        abs_coeff: f64,
        shift_coeff: f64,
        asym: f64,
    ) -> Self {
        let inter_coeff = scat_coeff + abs_coeff + shift_coeff;
        let albedo = 1.0 - (abs_coeff / inter_coeff);
        let shift_prob = shift_coeff / inter_coeff;

        Self {
            ref_index,
            inter_coeff,
            albedo,
            shift_prob,
            asym,
        }
    }

    /// Get the refractive index.
    #[post(ret >= 1.0)]
    pub fn ref_index(&self) -> f64 {
        self.ref_index
    }

    /// Get the interaction coefficient. [m^-1]
    #[post(ret > 0.0)]
    pub fn inter_coeff(&self) -> f64 {
        self.inter_coeff
    }

    /// Get the single scattering albedo.
    #[post(ret > 0.0)]
    #[post(ret <= 1.0)]
    pub fn albedo(&self) -> f64 {
        self.albedo
    }

    /// Get the shift probability.
    #[post(ret >= 0.0)]
    #[post(ret < 1.0)]
    pub fn shift_prob(&self) -> f64 {
        self.shift_prob
    }

    /// Get the asymmetry parameter.
    #[post(ret > -1.0)]
    #[post(ret < 1.0)]
    pub fn asym(&self) -> f64 {
        self.asym
    }
}
