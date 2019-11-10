//! Transform trait.

use nalgebra::Isometry3;

/// Geometries implementing this trait may be transformed.
pub trait Transform {
    /// Apply the given transformation.
    fn trans(&mut self, trans: &Isometry3<f64>);
}
