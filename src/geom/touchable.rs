//! Collision trait.

use contracts::post;
use nalgebra::Point3;

/// Geometry measurement trait.
/// Structures implementing this trait can be tested measured using a distance function.
pub trait Touchable {
    /// Determine the closest contained point to the given point.
    fn closest_point(&self, p: &Point3<f64>) -> Point3<f64>;

    /// Determine the distance from the given point to the structure.
    #[post(ret > 0.0)]
    fn dist(&self, p: &Point3<f64>) -> f64 {
        nalgebra::distance(&self.closest_point(&p), &p)
    }
}
