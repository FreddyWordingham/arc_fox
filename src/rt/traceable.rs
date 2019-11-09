//! Ray-traceable trait.

use super::Ray;
use nalgebra::{Unit, Vector3};

/// Types implementing this trait can be traced using a ray.
pub trait Traceable {
    /// Determine if a ray hit occurs at all.
    fn hit(&self, ray: &Ray) -> bool;

    /// Distance to the surface along the ray's line of travel.
    fn dist(&self, ray: &Ray) -> Option<f64>;

    /// Distance to the surface along the ray's line of travel and normal unit vector at the point of collision.
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)>;
}
