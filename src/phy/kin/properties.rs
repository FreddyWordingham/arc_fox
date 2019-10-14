//! Kinetic statistical properties structure.

use contracts::pre;

/// Kinetic statistical physical properties.
/// Contains parameters governing how number density changes over space with time.
#[derive(Debug)]
pub struct Properties {
    /// Diffusion coefficient [m^2/t].
    diff_coeff: f64,
}

impl Properties {
    /// Construct a new instance.
    #[pre(0.0 < diff_coeff)]
    pub fn new(diff_coeff: f64) -> Self {
        Self { diff_coeff }
    }

    /// Get the diffusion coefficient.
    pub fn diff_coeff(&self) -> f64 {
        self.diff_coeff
    }
}
