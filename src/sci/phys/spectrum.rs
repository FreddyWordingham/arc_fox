//! Spectrum enumeration.

use contracts::{post, pre};
use rand::rngs::ThreadRng;

/// Spectrum enumeration implementation.
#[derive(Debug)]
pub enum Spectrum {
    /// Single wavelength.
    Laser(f64),
}

impl Spectrum {
    /// Construct a new laser spectrum.
    #[pre(wavelength > 0.0)]
    pub fn new_laser(wavelength: f64) -> Self {
        Spectrum::Laser(wavelength)
    }

    /// Sample the spectrum for a wavelength.
    #[post(ret > 0.0)]
    pub fn sample(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Spectrum::Laser(w) => *w,
        }
    }
}
