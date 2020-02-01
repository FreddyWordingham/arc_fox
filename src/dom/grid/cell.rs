//! Cell structure implementation.

use crate::{
    access,
    dom::{Name, Set},
    geom::{Aabb, Collide, Mesh, SmoothTriangle},
    uni::{Interface, State},
};

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
}
