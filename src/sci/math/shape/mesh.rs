//! Triangular-mesh structure.

use crate::{
    sci::{
        math::{
            geom::{Collide, Transform},
            rt::{Ray, Trace},
            Normal,
        },
        Aabb, MeshBuilder, Triangle,
    },
    util::list::alphabet::Greek::Alpha,
};
use contracts::{post, pre};
use nalgebra::{Similarity3, Unit, Vector3};
use std::collections::HashMap;

/// Mesh structure implementation.
/// Forms the surface of the majority of complex components.
#[derive(Debug, Clone)]
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

    /// Build a new instance.
    pub fn build(builder: MeshBuilder, meshes: &HashMap<String, Self>) -> Self {
        for (name, mesh) in meshes.iter() {
            if name == &builder.name {
                let mut base = mesh.clone();

                if let Some(transform_builder) = builder.trans {
                    base.transform(&transform_builder.build());
                }

                return base;
            }
        }

        panic!("Could not find required mesh.");
    }

    /// Initialise the axis-aligned bounding box for the given list of triangles.
    #[pre(!tris.is_empty())]
    fn init_aabb(tris: &[Triangle]) -> Aabb {
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

    /// Reference the bounding box.
    pub const fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Reference the list of component triangles.
    pub const fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }
}

impl Transform for Mesh {
    fn transform(&mut self, trans: &Similarity3<f64>) {
        for tri in &mut self.tris {
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

        for tri in &self.tris {
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
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().1.is_normal())))]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_norm(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[post(ret.is_none() || ret.unwrap().0 > 0.0)]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_inside(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    #[post(ret.is_none() || (ret.unwrap().0 > 0.0 && (ret.unwrap().2.is_normal())))]
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
