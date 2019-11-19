//! Material structure.

use crate::{json, math::Formula, opt::Environment, util::Range};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Material structure implementation.
/// Stores the local optical, diffusive and kinematic information.
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
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
}

impl Material {
    /// Construct a new instance.
    #[pre(range.min() > 0.0)]
    #[pre(visc.is_none() || visc.unwrap() > 0.0)]
    pub fn new(
        range: Range,
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
        visc: Option<f64>,
    ) -> Self {
        Self {
            range,
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
            visc,
        }
    }

    /// Get the optical environment for a given wavelength.
    #[pre(self.range.contains(w))]
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.res(w),
            self.scat_coeff.res(w),
            self.abs_coeff.res(w),
            self.shift_coeff.res(w),
            self.asym.res(w),
        )
    }

    /// Optional viscosity. [kg m s^-1]
    pub fn visc(&self) -> Option<f64> {
        self.visc
    }
}

json!(Material);
