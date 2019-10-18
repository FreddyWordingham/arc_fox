//! Surface collidable trait.

use super::Ray;
use nalgebra::{Unit, Vector3};

/// Types implementing this trait can be observed using a ray.
pub trait Surface {
    /// Determine if an interaction occurs at all.
    fn intersect(&self, ray: &Ray) -> bool;

    /// Distance to the surface along the ray's line of travel.
    fn distance(&self, ray: &Ray) -> Option<f64>;

    /// Distance to the surface along the ray's line of travel and normal unit vector at the point of collision.
    fn distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)>;
}
