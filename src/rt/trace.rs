//! Trace trait.

use super::Ray;
use nalgebra::{Point3, Unit, Vector3};
use std::fmt::Debug;

/// Trace trait implementation.
/// Types implementing this trait can be traced using rays.
pub trait Trace: Debug {
    /// Determine if a ray hit occurs at all.
    fn hit(&self, ray: &Ray) -> bool;

    /// Distance to the surface along the ray's line of travel.
    fn dist(&self, ray: &Ray) -> Option<f64>;

    /// Distance to the surface along the ray's line of travel and normal unit vector at the point of collision.
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)>;

    /// Distance to the surface along the ray's line of travel and side of collision.
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)>;

    /// Calculate the hit point of a ray.
    fn hit_point(&self, ray: &Ray) -> Option<Point3<f64>> {
        return if let Some(dist) = self.dist(ray) {
            let mut scan = ray.clone();
            scan.travel(dist);
            Some(*scan.pos())
        } else {
            None
        };
    }
}
