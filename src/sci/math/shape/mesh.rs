//! Triangular-mesh structure.

use super::{
    super::{
        geom::{Collide, Transform},
        rt::{Ray, Trace},
        Normal,
    },
    Aabb, Triangle,
};
use crate::util::list::alphabet::Greek::Alpha;
use contracts::{post, pre};
use nalgebra::{Similarity3, Unit, Vector3};

/// Mesh structure implementation.
/// Forms the surface of the majority of complex components.
#[derive(Debug)]
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<Triangle>,
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

    /// Initialise the axis-aligned bounding box for the given list of triangles.
    #[pre(!tris.is_empty())]
    fn init_aabb(tris: &Vec<Triangle>) -> Aabb {
        let mut mins = tris[0].verts()[Alpha as usize];
        let mut maxs = mins;

        for tri in tris.iter() {
            for v in tri.verts().iter() {
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

    /// Reference the list of component triangles.
    pub fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }
}

impl Transform for Mesh {
    fn transform(&mut self, trans: &Similarity3<f64>) {
        for tri in self.tris.iter_mut() {
            tri.transform(trans);
        }

        self.aabb = Self::init_aabb(&self.tris);
    }
}

impl Collide for Mesh {
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

impl Trace for Mesh {
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    #[post(ret.is_none() || ret.unwrap() > 0.0)]
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

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().1.is_normal())))]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .map(|tri| tri.dist_norm(ray))
            .filter(|dist_norm| dist_norm.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[post(ret.is_none() || ret.unwrap().0 > 0.0)]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .map(|tri| tri.dist_inside(ray))
            .filter(|dist_inside| dist_inside.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().2.is_normal())))]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .map(|tri| tri.dist_inside_norm(ray))
            .filter(|dist_inside_norm| dist_inside_norm.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
