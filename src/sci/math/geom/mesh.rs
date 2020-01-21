//! Triangular-mesh structure.

use crate::{
    access,
    sci::math::{
        geom::{Aabb, Collide, SmoothTriangle},
        rt::{Ray, Trace},
    },
    util::list::alphabet::Greek::Alpha,
};
use nalgebra::{Unit, Vector3};

/// Mesh geometry.
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
}

impl Mesh {
    access!(aabb, Aabb);
    access!(tris, Vec<SmoothTriangle>);

    /// Construct a new instance.
    pub fn new(tris: Vec<SmoothTriangle>) -> Self {
        let mut mins = tris[0].tri().verts()[Alpha as usize];
        let mut maxs = mins;

        for tri in tris.iter() {
            for v in tri.tri().verts().iter() {
                for i in 0..3 {
                    if mins[i] > v[i] {
                        mins[i] = v[i];
                    } else if maxs[i] < v[i] {
                        maxs[i] = v[i];
                    }
                }
            }
        }

        let aabb = Aabb::new(mins, maxs);

        Self { aabb, tris }
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let mut area = 0.0;

        for tri in &self.tris {
            area += tri.tri().area();
        }

        area
    }
}

impl Collide for Mesh {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        if !self.aabb.overlap(aabb) {
            return false;
        }

        for tri in &self.tris {
            if tri.overlap(aabb) {
                return true;
            }
        }

        false
    }
}

impl Trace for Mesh {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_norm(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_inside(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_inside_norm(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
