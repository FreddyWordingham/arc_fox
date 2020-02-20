//! Cell structure implementation.

use crate::{
    access,
    dom::{Name, Set},
    geom::{Aabb, Collide, Mesh, Ray, SmoothTriangle, Trace},
    uni::{Interface, State},
};
use nalgebra::{Unit, Vector3};

/// Cell holding local information.
pub struct Cell<'a> {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: Name,
    /// Intersecting interface triangles.
    inter_tris: Vec<((&'a Name, &'a Interface), Vec<&'a SmoothTriangle>)>,
    /// Local chemical state.
    state: State,
}

impl<'a> Cell<'a> {
    access!(bound, Aabb);
    access!(mat, Name);
    access!(
        inter_tris,
        Vec<((&'a Name, &'a Interface), Vec<&'a SmoothTriangle>)>
    );
    access!(state, state_mut, State);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        bound: Aabb,
        mat: Name,
        inters: &'a Set<Interface>,
        meshes: &'a Set<Mesh>,
        state: State,
    ) -> Self {
        let mut inter_tris = Vec::new();

        for (name, inter) in inters.map() {
            let mesh = meshes.map().get(inter.surf()).expect("Invalid mesh name.");
            if bound.overlap(mesh.aabb()) {
                let mut intersections = Vec::new();
                for tri in mesh.tris().iter().filter(|tri| tri.overlap(&bound)) {
                    intersections.push(tri);
                }

                if !intersections.is_empty() {
                    inter_tris.push(((name, inter), intersections));
                }
            }
        }

        Self {
            bound,
            mat,
            inter_tris,
            state,
        }
    }

    /// Determine the distance to the next interface along a ray's line of sight.
    #[inline]
    #[must_use]
    pub fn inter_dist(&self, ray: &Ray) -> Option<f64> {
        assert!(self.bound().contains(ray.pos()));

        let mut nearest = None;
        for ((_name, _inter), tris) in &self.inter_tris {
            for tri in tris {
                if let Some(dist) = tri.dist(ray) {
                    if nearest.is_none() || dist < nearest.expect("Something went wrong...") {
                        nearest = Some(dist);
                    }
                }
            }
        }

        nearest
    }

    /// Determine the distance to an interface contained within the cell, if hitting on the inside of the interface, and the normal at the intersection point.
    #[inline]
    #[must_use]
    pub fn inter_dist_inside_norm_inter(
        &self,
        ray: &Ray,
    ) -> Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> {
        let mut nearest: Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> = None;

        for ((_name, inter), tris) in &self.inter_tris {
            for tri in tris {
                if let Some((dist, inside, norm)) = tri.dist_inside_norm(ray) {
                    if nearest.is_none() || dist < nearest.expect("Something went wrong...").0 {
                        nearest = Some((dist, inside, norm, inter));
                    }
                }
            }
        }

        nearest
    }
}
