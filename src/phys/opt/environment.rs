//! Optical environment structure.

use contracts::pre;

/// Information about an optical.
pub struct Environment {
    /// Refractive index.
    pub ref_index: f64,
    /// Interaction coefficient. [m^-1]
    pub inter_coeff: f64,
    /// Single scattering albedo.
    pub albedo: f64,
    /// Asymmetry parameter.
    pub asym: f64,
}

impl Environment {
    /// Construct a new instance.
    #[pre(ref_index > 0.0)]
    #[pre(scat_coeff > 0.0)]
    #[pre(abs_coeff >= 0.0)]
    #[pre(shift_coeff >= 0.0)]
    #[pre(asym >= -1.0)]
    #[pre(asym <= 1.0)]
    pub fn new(
        ref_index: f64,
        scat_coeff: f64,
        abs_coeff: f64,
        shift_coeff: f64,
        asym: f64,
    ) -> Self {
        let inter_coeff = scat_coeff + abs_coeff + shift_coeff;
        let albedo = abs_coeff / inter_coeff;

        Self {
            ref_index,
            inter_coeff,
            albedo,
            asym,
        }
    }
}