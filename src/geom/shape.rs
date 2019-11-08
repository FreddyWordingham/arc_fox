//! Shape enumeration.

use crate::{
    dom::Aabb,
    rt::{Ray, Traceable},
};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use std::f64::{INFINITY, NEG_INFINITY};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Shape enumeration.
/// Used to compose entities.
#[derive(Debug)]
pub enum Shape {
    /// Plane shape.
    Plane {
        /// Point on the plane.
        pos: Point3<f64>,
        /// Normal.
        norm: Unit<Vector3<f64>>,
    },
    /// Sphere shape.
    Sphere {
        /// Centre of the sphere.
        pos: Point3<f64>,
        /// Radius.
        rad: f64,
    },
}

impl Shape {
    /// Construct a new Plane instance.
    pub fn new_plane(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self::Plane { pos, norm }
    }

    /// Construct a new Sphere instance.
    #[pre(rad > 0.0)]
    pub fn new_sphere(pos: Point3<f64>, rad: f64) -> Self {
        Self::Sphere { pos, rad }
    }

    /// Create a bounding box for the shape.
    pub fn aabb(&self) -> Aabb {
        match self {
            Shape::Plane { pos: _, norm: _ } => Aabb::new(
                Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
                Point3::new(INFINITY, INFINITY, INFINITY),
            ),
            Shape::Sphere { pos, rad } => {
                // TODO
                Aabb::new(
                    Point3::new(pos.x - rad, pos.y - rad, pos.z - rad),
                    Point3::new(pos.x + rad, pos.y + rad, pos.z + rad),
                )
            }
        }
    }

    /// Get a list of the component shapes.
    pub fn components(&self) -> Vec<&Shape> {
        match self {
            Shape::Plane { pos: _, norm: _ } => vec![&self],
            Shape::Sphere { pos: _, rad: _ } => vec![&self],
        }
    }
}

impl Traceable for Shape {
    fn hit(&self, ray: &Ray) -> bool {
        match self {
            Shape::Plane { pos, norm } => {
                let d = norm.dot(&ray.dir);

                if d.abs() > EPSILON {
                    let po = pos - ray.pos;
                    let dist = po.dot(norm) / d;

                    return dist >= 0.0;
                }

                false
            }
            Shape::Sphere { pos, rad } => {
                let oc = ray.pos - pos;
                let b = ray.dir.dot(&oc).powi(2);
                let c = oc.magnitude().powi(2) - rad.powi(2);

                b >= c
            }
        }
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
            Shape::Sphere { pos, rad } => {
                let oc = ray.pos - pos;
                let a = -ray.dir.dot(&oc);
                let b = ray.dir.dot(&oc).powi(2);
                let c = oc.magnitude().powi(2) - rad.powi(2);

                if b < c {
                    return None;
                }

                let det_sqrt = (b - c).sqrt();

                let d0 = a + det_sqrt;
                let d1 = a - det_sqrt;

                if (d0 < 0.0) && (d1 < 0.0) {
                    return None;
                }

                if d0 < 0.0 {
                    if d1 >= 0.0 {
                        return Some(d1);
                    }

                    return None;
                }

                if d1 < 0.0 {
                    if d0 >= 0.0 {
                        return Some(d0);
                    }

                    return None;
                }

                if d0 < d1 {
                    return Some(d0);
                }

                Some(d1)
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
            Shape::Sphere { pos, rad: _ } => {
                if let Some(dist) = self.dist(ray) {
                    let p = ray.pos + (ray.dir.as_ref() * dist);
                    let norm = Unit::new_normalize(p - pos);

                    return Some((dist, norm));
                }

                None
            }
        };
    }
}
