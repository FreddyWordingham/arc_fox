//! Optical property structure.

use crate::{access, sci::math::Lambda, sci::phys::Environment};

/// Optical properties structure.
pub struct Optics {
    /// Refractive index.
    ref_index: Lambda,
    /// Scattering coefficient. [m^-1]
    scat_coeff: Lambda,
    /// Absorption coefficient. [m^-1]
    abs_coeff: Lambda,
    /// Shift coefficient. [m^-1]
    shift_coeff: Lambda,
    /// Asymmetry parameter.
    asym: Lambda,
}

impl Optics {
    access!(ref_index, Lambda);
    access!(scat_coeff, Lambda);
    access!(abs_coeff, Lambda);
    access!(shift_coeff, Lambda);
    access!(asym, Lambda);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        ref_index: Lambda,
        scat_coeff: Lambda,
        abs_coeff: Lambda,
        shift_coeff: Lambda,
        asym: Lambda,
    ) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
        }
    }

    /// Determine the local optical properties for a given wavelength.
    #[inline]
    #[must_use]
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.y(w),
            self.scat_coeff.y(w),
            self.abs_coeff.y(w),
            self.shift_coeff.y(w),
            self.asym.y(w),
        )
    }
}
