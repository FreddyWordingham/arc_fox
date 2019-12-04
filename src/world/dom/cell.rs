//! Cell structure.

use crate::{
    sci::math::{
        geom::Collide,
        rt::Ray,
        shape::{Aabb, Triangle},
    },
    world::{
        mat::{Interface, Material},
        parts::interfaces,
    },
};
use nalgebra::Unit;

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

        let mut ray = None;
        let pos = boundary.centre();
        for inter in interfaces {
            for tri in inter.mesh().tris() {
                let tar = tri.centre();
                if domain.contains(&tar) {
                    let dir = Unit::new_normalize(tar - pos);

                    if tri.plane_norm().dot(&dir) > 1.0e-3 {
                        ray = Some(Ray::new(pos, dir));
                        break;
                    }
                }
            }
        }
        let ray = ray.unwrap();

        let (_dist, inside, inter) =
            interfaces::dist_inside_inter(&ray, domain, interfaces).unwrap();
        let mat = if inside {
            inter.in_mat()
        } else {
            inter.out_mat()
        };

        Self {
            boundary,
            inter_tris,
            mat,
        }
    }

    /// Reference the boundary.
    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }

    /// Intersecting interface triangles.
    pub fn inter_tris(&self) -> &Vec<(&'a Interface<'a>, Vec<&'a Triangle>)> {
        &self.inter_tris
    }

    /// Central material.
    pub fn mat(&self) -> &'a Material {
        self.mat
    }
}
