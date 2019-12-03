//! Cell structure.

use crate::{
    sci::math::{
        geom::Collide,
        rt::{Ray, Trace},
        shape::{Aabb, Triangle},
    },
    world::mat::{Interface, Material},
};
use contracts::pre;
use nalgebra::{Point3, Unit};

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
        let mut tar = None;
        for (_inter, tris) in inter_tris {
            for tri in tris {
                if let Some(point) = tri.union_point(boundary) {
                    tar = Some(point);
                    break;
                }
            }
        }
        let tar = tar.unwrap();

        let ray = Ray::new(*pos, Unit::new_normalize(tar - pos));

        let mut nearest: Option<(f64, bool, &Interface)> = None;
        for (inter, tris) in inter_tris {
            for tri in tris {
                if let Some((dist, inside)) = tri.dist_inside(&ray) {
                    if nearest.is_none() || nearest.unwrap().0 > dist {
                        nearest = Some((dist, inside, inter));
                    }
                }
            }
        }

        if let Some((dist, inside, inter)) = nearest {
            if dist <= boundary.dist(&ray).unwrap() {
                return if inside {
                    inter.in_mat()
                } else {
                    inter.out_mat()
                };
            }
        }

        panic!("Unable to observe material within the cell.");
    }

    /// Determine the material from the interfaces.
    #[pre(domain.contains(&pos))]
    pub fn mat_at_pos_from_interface_list(
        pos: &Point3<f64>,
        domain: &Aabb,
        interfaces: &'a [Interface],
    ) -> &'a Material {
        let mut tar = None;
        for inter in interfaces {
            for tri in inter.mesh().tris() {
                if let Some(point) = tri.union_point(domain) {
                    tar = Some(point);
                    break;
                }
            }
        }
        let tar = tar.unwrap();

        let ray = Ray::new(*pos, Unit::new_normalize(tar - pos));

        let mut nearest: Option<(f64, bool, &Interface)> = None;
        for inter in interfaces {
            if let Some((dist, inside)) = inter.mesh().dist_inside(&ray) {
                if nearest.is_none() || nearest.unwrap().0 > dist {
                    nearest = Some((dist, inside, inter));
                }
            }
        }

        if let Some((dist, inside, inter)) = nearest {
            if dist <= domain.dist(&ray).unwrap() {
                return if inside {
                    inter.in_mat()
                } else {
                    inter.out_mat()
                };
            }
        }

        panic!("Unable to observe material within the domain.");
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
        &self.mat
    }
}
