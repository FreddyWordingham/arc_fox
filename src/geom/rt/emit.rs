//! Trace trait.

use crate::geom::Ray;
use rand::rngs::ThreadRng;

/// Emit trait implementation.
/// Types implementing this trait can emit rays.
pub trait Emit {
    fn emit(rng: &mut ThreadRng) -> Ray;
}
