//! Light structure.

use crate::{opt::Photon, rt::Emitter};
use contracts::pre;
use rand::rngs::ThreadRng;

/// Photon emission structure.
pub struct Light {
    /// Emission surface.
    emit: Box<dyn Emitter>,
    /// Emission wavelength.
    wavelength: f64,
    /// Power. [J/s]
    power: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(power > 0.0)]
    #[pre(wavelength > 0.0)]
    pub fn new(emit: Box<dyn Emitter>, wavelength: f64, power: f64) -> Self {
        Self {
            emit,
            wavelength,
            power,
        }
    }

    /// Emit a new photon.
    #[pre(total_phot > 0)]
    pub fn emit(&self, rng: &mut ThreadRng, total_phot: u64) -> Photon {
        Photon::new(
            self.emit.emit(rng),
            self.power / total_phot as f64,
            self.wavelength,
        )
    }
}
