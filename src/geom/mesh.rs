//! Triangle mesh structure.

use super::{Aabb, Collision, Transform, Triangle};
use crate::{
    dim::Greek::Alpha,
    rt::{Ray, Traceable},
};
use contracts::pre;
use nalgebra::{Similarity3, Unit, Vector3};

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
        Self {
            aabb: Self::init_aabb(&tris),
            tris,
        }
    }

    /// Determine the axis-aligned bounding box for the given list of triangles.
    fn init_aabb(tris: &Vec<Triangle>) -> Aabb {
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

        Aabb::new(mins, maxs)
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

impl Transform for Mesh {
    fn trans(&mut self, trans: &Similarity3<f64>) {
        for tri in self.tris.iter_mut() {
            tri.trans(trans)
        }

        self.aabb = Self::init_aabb(&self.tris);
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
