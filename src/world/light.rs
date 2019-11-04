//! Light structure.

use crate::rt::Emitter;
use contracts::pre;

/// Photon emission structure.
pub struct Light {
    /// Emission surface.
    emit: Emitter,
    /// Power. [J/s]
    pow: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(pow > 0.0)]
    pub fn new(emit: Emitter, pow: f64) -> Self {
        Self {
            emit,
            pow
        }
    }
}
