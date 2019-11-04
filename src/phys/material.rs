//! Physical material structure.

use super::opt::Environment;
use crate::{math::Formula, util::Range};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Physical material structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /// Range of valid wavelengths.
    range: Range,
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

impl Material {
    /// Construct a new instance.
    #[pre(range.min() > 0.0)]
    pub fn new(
        range: Range,
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
    ) -> Self {
        Self {
            range,
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
        }
    }

    /// Get the refractive index at the given wavelength.
    #[pre(self.range.contains(w))]
    pub fn ref_index(&self, w: f64) -> f64 {
        self.ref_index.calc(w)
    }

    /// Get the scattering coefficient at the given wavelength.
    #[pre(self.range.contains(w))]
    pub fn scat_coeff(&self, w: f64) -> f64 {
        self.scat_coeff.calc(w)
    }

    /// Get the absorption coefficient at the given wavelength.
    #[pre(self.range.contains(w))]
    pub fn abs_coeff(&self, w: f64) -> f64 {
        self.abs_coeff.calc(w)
    }

    /// Get the shift coefficient at the given wavelength.
    #[pre(self.range.contains(w))]
    pub fn shift_coeff(&self, w: f64) -> f64 {
        self.shift_coeff.calc(w)
    }

    /// Get the refractive index at the given wavelength.
    #[pre(self.range.contains(w))]
    pub fn asym(&self, w: f64) -> f64 {
        self.asym.calc(w)
    }

    /// Get the optical environment for a given wavelength.
    #[pre(self.range.contains(w))]
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.calc(w),
            self.scat_coeff.calc(w),
            self.abs_coeff.calc(w),
            self.shift_coeff.calc(w),
            self.asym.calc(w),
        )
    }
}
