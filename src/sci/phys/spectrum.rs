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
    pub fn new_laser(wavelength: f64) -> Self {
        Self::Laser(wavelength)
    }

    /// Sample the spectrum for a wavelength.
    #[inline]
    pub fn sample(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Laser(w) => *w,
        }
    }
}
