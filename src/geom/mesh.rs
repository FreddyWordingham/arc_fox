//! Triangle mesh structure.

use super::{Aabb, Triangle};
use crate::rt::{Ray, Traceable};
use contracts::pre;
use nalgebra::{Unit, Vector3};

/// Triangle mesh structure.
pub struct Mesh {
    /// List of component triangles.
    tris: Vec<Triangle>,
    /// Bounding box.
    aabb: Aabb,
}

impl Mesh {
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>, aabb: Aabb) -> Self {
        let mut mins = tris[0].verts()[0];
        let mut maxs = mins;
        for t in tris.iter() {
            for v in t.verts().iter() {
                for i in 0..3 {
                    if mins[i] > v[i] {
                        mins[i] = v[i];
                    } else if maxs[i] < v[i] {
                        maxs[i] = v[i];
                    }
                }
            }
        }

        Self {
            tris,
            aabb: Aabb::new(mins, maxs),
        }
    }
}

impl Traceable for Mesh {
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        for t in self.tris {
            if t.hit(ray) {
                return true;
            }
        }

        false
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.hit(ray) {
            return None;
        }

        let ret = None;
        for t in self.tris {
            if let Some(dist) = t.dist(ray) {
                if ret.is_none() || dist < ret.unwrap() {
                    ret = Some(dist);
                }
            }
        }

        ret
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        let ret: Option<(f64, Unit<Vector3<f64>>)> = None;
        for t in self.tris {
            if let Some((dist, norm)) = t.dist_norm(ray) {
                if ret.is_none() || dist < ret.unwrap().0 {
                    ret = Some((dist, norm));
                }
            }
        }

        ret
    }
}
