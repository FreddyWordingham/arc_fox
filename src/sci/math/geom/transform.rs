//! Transform trait.

use nalgebra::Similarity3;
use std::fmt::Debug;

/// Types implementing this trait may be transformed.
pub trait Transform: Debug {
    /// Apply the given transformation.
    fn transform(&mut self, trans: &Similarity3<f64>);
}
