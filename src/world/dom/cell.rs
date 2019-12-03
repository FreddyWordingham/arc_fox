//! Cell structure.

use crate::{
    sci::math::{
        geom::Collide,
        shape::{Aabb, Triangle},
    },
    world::mat::{Interface, Material},
};
use nalgebra::{Point3};
use contracts::pre;

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, domain: &Aabb, interfaces: &'a [Interface]) -> Self {
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

        let mat = if inter_tris.is_empty() {
            Self::mat_at_pos_from_interface_list(&boundary.centre(), domain, interfaces)
        } else {
            Self::mat_at_pos_from_sub_tri_list(&boundary.centre(), &boundary, &inter_tris)
        };

        Self {
            boundary,
            inter_tris,
            mat,
        }
    }

    /// Determine the material from the triangle sublist.
    #[pre(boundary.contains(&pos))]
    pub fn mat_at_pos_from_sub_tri_list(
        pos: &Point3<f64>,
        boundary: &Aabb,
        inter_tris: &[(&'a Interface<'a>, Vec<&'a Triangle>)],
    ) -> &'a Material {
        inter_tris[0].0.in_mat() // TODO
    }

    /// Determine the material from the interfaces.
    #[pre(domain.contains(&pos))]
    pub fn mat_at_pos_from_interface_list(
        pos: &Point3<f64>,
        domain: &Aabb,
        interfaces: &'a [Interface],
    ) -> &'a Material {
        interfaces[0].out_mat() // TODO
    }
}
