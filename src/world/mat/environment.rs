//! Environment structure.

use contracts::pre;

/// Environment structure implementation.
/// Information about the local optical environment.
#[derive(Debug, Clone)]
pub struct Environment {
    /// Refractive index.
    pub ref_index: f64,
    /// Interaction coefficient. [m^-1]
    pub inter_coeff: f64,
    /// Single scattering albedo.
    pub albedo: f64,
    /// Shift probability.
    pub shift_prob: f64,
    /// Asymmetry parameter.
    pub asym: f64,
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
}
