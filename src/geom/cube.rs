//! Geometric cube structure.

use super::{Ray, Surface, Triangle};
use crate::util::Container;
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};

/// Axis-aligned box.
/// Commonly used to partition the domain.
#[derive(Debug)]
pub struct Cube {
    /// Minimum bound.
    mins: Point3<f64>,
    /// Maximum bound.
    maxs: Point3<f64>,
}

impl Cube {
    /// Construct a new instance.
    #[pre(mins < maxs)]
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Reference the minimum bound.
    pub fn mins(&self) -> &Point3<f64> {
        &self.mins
    }

    /// Reference the maximum bound.
    pub fn maxs(&self) -> &Point3<f64> {
        &self.maxs
    }

    /// Calculate the widths.
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    pub fn half_widths(&self) -> Vector3<f64> {
        (self.maxs - self.mins) / 2.0
    }

    /// Calculate the centre position.
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Determine if the point is contained.
    /// Points lying exactly at the surface are considered contained.
    pub fn contained(&self, point: Point3<f64>) -> bool {
        (self.mins.x <= point.x)
            && (point.x <= self.maxs.x)
            && (self.mins.y <= point.y)
            && (point.y <= self.maxs.y)
            && (self.mins.z <= point.z)
            && (point.z <= self.maxs.z)
    }

    /// Determine if a triangle collides with the cube.
    pub fn collides(&self, tri: &Triangle) -> bool {
        let c = self.centre();
        let e = self.half_widths();

        let v0 = tri.verts()[0] - c;
        let v1 = tri.verts()[1] - c;
        let v2 = tri.verts()[2] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vector3::x_axis();
        let u1 = Vector3::y_axis();
        let u2 = Vector3::z_axis();

        let axis_test = |axis: &Vector3<f64>| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);

            let r = (e.x * (u0.dot(axis)).abs())
                + (e.y * (u1.dot(axis)).abs())
                + (e.z * (u2.dot(axis)).abs());

            if (-(p0.max(p1).max(p2))).max(p0.min(p1).min(p2)) > r {
                return false;
            }

            true
        };

        if !axis_test(&u0) {
            return false;
        }
        if !axis_test(&u1) {
            return false;
        }
        if !axis_test(&u2) {
            return false;
        }

        let axis_u0_f0 = u0.cross(&f0);
        let axis_u0_f1 = u0.cross(&f1);
        let axis_u0_f2 = u0.cross(&f2);

        let axis_u1_f0 = u1.cross(&f0);
        let axis_u1_f1 = u1.cross(&f1);
        let axis_u1_f2 = u1.cross(&f2);

        let axis_u2_f0 = u2.cross(&f0);
        let axis_u2_f1 = u2.cross(&f1);
        let axis_u2_f2 = u2.cross(&f2);

        if !axis_test(&axis_u0_f0) {
            return false;
        }
        if !axis_test(&axis_u0_f1) {
            return false;
        }
        if !axis_test(&axis_u0_f2) {
            return false;
        }

        if !axis_test(&axis_u1_f0) {
            return false;
        }
        if !axis_test(&axis_u1_f1) {
            return false;
        }
        if !axis_test(&axis_u1_f2) {
            return false;
        }

        if !axis_test(&axis_u2_f0) {
            return false;
        }
        if !axis_test(&axis_u2_f1) {
            return false;
        }
        if !axis_test(&axis_u2_f2) {
            return false;
        }

        if !axis_test(tri.normal()) {
            return false;
        }

        true
    }
}

impl Surface for Cube {
    fn intersect(&self, ray: &Ray) -> bool {
        let dir_frac = Vector3::new(
            1.0 / ray.direction().x,
            1.0 / ray.direction().y,
            1.0 / ray.direction().z,
        );

        let tx0 = (self.mins.x - ray.origin().x) * dir_frac.x;
        let tx1 = (self.maxs.x - ray.origin().x) * dir_frac.x;
        let ty0 = (self.mins.y - ray.origin().y) * dir_frac.y;
        let ty1 = (self.maxs.y - ray.origin().y) * dir_frac.y;
        let tz0 = (self.mins.z - ray.origin().z) * dir_frac.z;
        let tz1 = (self.maxs.z - ray.origin().z) * dir_frac.z;

        let t_min = (tx0.min(tx1)).max(ty0.min(ty1)).max(tz0.min(tz1));
        let t_max = (tx0.max(tx1)).min(ty0.max(ty1)).min(tz0.max(tz1));

        !((t_max < 0.0) || (t_min > t_max))
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let dir_frac = Vector3::new(
            1.0 / ray.direction().x,
            1.0 / ray.direction().y,
            1.0 / ray.direction().z,
        );

        let t1 = (self.mins.x - ray.origin().x) * dir_frac.x;
        let t2 = (self.maxs.x - ray.origin().x) * dir_frac.x;
        let t3 = (self.mins.y - ray.origin().y) * dir_frac.y;
        let t4 = (self.maxs.y - ray.origin().y) * dir_frac.y;
        let t5 = (self.mins.z - ray.origin().z) * dir_frac.z;
        let t6 = (self.maxs.z - ray.origin().z) * dir_frac.z;

        let t_min = (t1.min(t2)).max(t3.min(t4)).max(t5.min(t6));
        let t_max = (t1.max(t2)).min(t3.max(t4)).min(t5.max(t6));

        if (t_max < 0.0) || (t_min > t_max) {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    fn distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let dir_frac = Vector3::new(
            1.0 / ray.direction().x,
            1.0 / ray.direction().y,
            1.0 / ray.direction().z,
        );

        let t1 = Container::new(
            (self.mins.x - ray.origin().x) * dir_frac.x,
            -Vector3::x_axis(),
        );
        let t2 = Container::new(
            (self.maxs.x - ray.origin().x) * dir_frac.x,
            Vector3::x_axis(),
        );
        let t3 = Container::new(
            (self.mins.y - ray.origin().y) * dir_frac.y,
            -Vector3::y_axis(),
        );
        let t4 = Container::new(
            (self.maxs.y - ray.origin().y) * dir_frac.y,
            Vector3::y_axis(),
        );
        let t5 = Container::new(
            (self.mins.z - ray.origin().z) * dir_frac.z,
            -Vector3::z_axis(),
        );
        let t6 = Container::new(
            (self.maxs.z - ray.origin().z) * dir_frac.z,
            Vector3::z_axis(),
        );

        let t_min = (&t1.min(&t2)).max(&t3.min(&t4)).max(&t5.min(&t6));
        let t_max = (&t1.max(&t2)).min(&t3.max(&t4)).min(&t5.max(&t6));

        if (t_max < 0.0) || (t_min > t_max) {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min.separate());
        }

        Some(t_max.separate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.mins, Point3::new(-1.0, 2.0, -3.14159));
        assert_eq!(cube.maxs, Point3::new(1.0, 4.0, 3.14159));
    }

    #[test]
    fn getters() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.mins(), &Point3::new(-1.0, 2.0, -3.14159));
        assert_eq!(cube.maxs(), &Point3::new(1.0, 4.0, 3.14159));
    }

    #[test]
    fn calculators() {
        let cube = Cube::new(
            Point3::new(-1.0, 2.0, -3.14159),
            Point3::new(1.0, 4.0, 3.14159),
        );

        assert_eq!(cube.widths(), Vector3::new(2.0, 2.0, 6.28318));
        assert_eq!(cube.half_widths(), Vector3::new(1.0, 1.0, 3.14159));
        assert_eq!(cube.centre(), Point3::new(0.0, 3.0, 0.0));
    }
}
