//! Container trait.

use nalgebra::Point3;

/// Types implementing the container trait can determine when a given point is inside of it.
pub trait Container {
    /// Determine if a surfaces contains a given point.
    fn contains(&self, p: &Point3<f64>) -> bool;
}
