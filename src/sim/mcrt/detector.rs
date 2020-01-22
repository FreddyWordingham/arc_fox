//! Detect trait.

pub mod ccd;

pub use self::ccd::*;

use crate::sci::{math::rt::Ray, phys::Photon};

/// Trait of structures which can detect photons.
pub trait Detect {
    /// Determine the distance to the detector.
    fn dist(&self, ray: &Ray) -> Option<f64>;

    /// Detect a photon.
    fn detect(&mut self, phot: &Photon);
}
