//! Triangle mesh structure.

use super::{super::Collision, Aabb, Triangle};
use crate::{
    dim::Greek::Alpha,
    rt::{Ray, Traceable},
};
use contracts::pre;
use nalgebra::{Unit, Vector3};

/// Triangle mesh surface.
pub struct Mesh {
    /// List of component triangles.
    tris: Vec<Triangle>,
    /// Bounding box.
    aabb: Aabb,
}

impl Mesh {
    /// Construct a new instance.
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>) -> Self {
        let mut mins = tris[0].verts()[Alpha as usize];
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

    /// Reference the list of triangles.
    pub fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }
}

impl Collision for Mesh {
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        if !self.aabb.overlap(aabb) {
            return false;
        }

        for tri in self.tris.iter() {
            if tri.overlap(aabb) {
                return true;
            }
        }

        false
    }
}

impl Traceable for Mesh {
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .map(|tri| tri.dist(ray))
            .filter(|dist| dist.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .map(|tri| tri.dist_norm(ray))
            .filter(|dist| dist.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
