//! Spectrum enumeration.

use rand::rngs::ThreadRng;

/// Spectrum enumeration implementation.
pub enum Spectrum {
    /// Single wavelength.
    Laser(f64),
}

impl Spectrum {
    /// Construct a new laser spectrum.
    #[inline]
    #[must_use]
    pub const fn new_laser(wavelength: f64) -> Self {
        Self::Laser { 0: wavelength }
    }

    /// Sample the spectrum for a wavelength.
    #[inline]
    #[must_use]
    pub fn sample(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Laser(w) => *w,
        }
    }
}
