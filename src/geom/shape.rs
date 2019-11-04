//! Shape enumeration.

use crate::{
    dom::Aabb,
    rt::{Ray, Traceable},
};
use nalgebra::{Point3, Unit, Vector3};
use std::f64::{INFINITY, NEG_INFINITY};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Shape enumeration.
/// Used to compose entities.
pub enum Shape {
    /// Plane shape.
    Plane {
        /// Point on the plane.
        pos: Point3<f64>,
        /// Normal.
        norm: Unit<Vector3<f64>>,
    },
}

impl Shape {
    /// Construct a new Plane instance.
    pub fn new_plane(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self::Plane { pos, norm }
    }

    /// Create a bounding box for the shape.
    pub fn aabb(&self) -> Aabb {
        match self {
            Shape::Plane { pos: _, norm: _ } => Aabb::new(
                Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
                Point3::new(INFINITY, INFINITY, INFINITY),
            ),
        }
    }
}

impl Traceable for Shape {
    fn hit(&self, ray: &Ray) -> bool {
        return match self {
            Shape::Plane { pos, norm } => {
                let d = norm.dot(&ray.dir);

                if d.abs() > EPSILON {
                    let po = pos - ray.pos;
                    let dist = po.dot(norm) / d;

                    return dist >= 0.0;
                }

                false
            }
        };
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        return match self {
            Shape::Plane { pos, norm } => {
                let d = norm.dot(&ray.dir);

                if d.abs() > EPSILON {
                    let rp = pos - ray.pos;
                    let dist = rp.dot(&norm) / d;

                    if dist < 0.0 {
                        return None;
                    }

                    return Some(dist);
                }

                None
            }
        };
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        return match self {
            Shape::Plane { pos, norm } => {
                let d = norm.dot(&ray.dir);

                if d.abs() > EPSILON {
                    let po = pos - ray.pos;
                    let dist = po.dot(&norm) / d;

                    if dist < 0.0 {
                        return None;
                    }

                    return Some((dist, *norm));
                }

                None
            }
        };
    }
}
