//! Cell structure.

use crate::{
    sci::math::{
        geom::Collide,
        shape::{Aabb, Triangle},
    },
    world::mat::Interface,
};

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    // /// Central material.
    // mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, interfaces: &'a [Interface]) -> Self {
        let mut inter_tris = Vec::new();
        for interface in interfaces {
            let mesh = interface.mesh();

            if mesh.overlap(&boundary) {
                let mut list = Vec::new();

                for tri in mesh.tris() {
                    if tri.overlap(&boundary) {
                        list.push(tri);
                    }
                }

                if !list.is_empty() {
                    inter_tris.push((interface, list));
                }
            }
        }

        Self {
            boundary,
            inter_tris,
        }
    }
}
