//! Light structure.

use crate::rt::Emitter;
use contracts::pre;

/// Photon emission structure.
pub struct Light {
    /// Emission surface.
    surf: Box<dyn Emitter>,
    /// Power. [J/s]
    pow: f64,
}

impl Light {
    /// Construct a new instance.
    #[pre(pow > 0.0)]
    pub fn new(surf: Box<dyn Emitter>, pow: f64) -> Self {
        Self {
            surf,
            pow
        }
    }
}
