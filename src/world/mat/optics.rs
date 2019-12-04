//! Optics structure.

use crate::{json, sci::math::op::Formula, world::mat::Environment};
use serde::{Deserialize, Serialize};

/// Optics structure implementation.
/// Holds the optical properties of a material.
#[derive(Debug, Deserialize, Serialize)]
pub struct Optics {
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient. [m^-1]
    scat_coeff: Formula,
    /// Absorption coefficient. [m^-1]
    abs_coeff: Formula,
    /// Shift coefficient. [m^-1]
    shift_coeff: Formula,
    /// Asymmetry parameter.
    asym: Formula,
}

impl Optics {
    /// Construct a new instance.
    pub fn new(
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
    ) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
        }
    }

    /// Get the optical environment for a given wavelength.
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.res(w),
            self.scat_coeff.res(w),
            self.abs_coeff.res(w),
            self.shift_coeff.res(w),
            self.asym.res(w),
        )
    }
}

json!(Optics);
