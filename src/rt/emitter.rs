//! Ray emitter trait.

use rand::rngs::ThreadRng;
use super::Ray;

/// Types implementing this trait can emit rays.
pub trait Emitter {
    /// Emit a new ray.
    fn emit(&self, rng: &mut ThreadRng) -> &Ray;
}
