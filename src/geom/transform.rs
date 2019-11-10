//! Transform trait.

use nalgebra::Similarity3;

/// Geometries implementing this trait may be transformed.
pub trait Transform {
    /// Apply the given transformation.
    fn trans(&mut self, trans: &Similarity3<f64>);
}
