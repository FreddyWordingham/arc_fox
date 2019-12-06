//! Cell structure.

use crate::{
    sci::math::{
        geom::Collide,
        rt::{Ray, Trace},
        shape::{Aabb, Triangle},
    },
    world::{
        dom::State,
        mat::{Interface, Material},
        parts::interfaces,
    },
};
use contracts::pre;
use nalgebra::Point3;
use nalgebra::{Unit, Vector3};

/// Cell structure implementation.
#[derive(Debug)]
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting interface triangles.
    inter_tris: Vec<(&'a Interface<'a>, Vec<&'a Triangle>)>,
    /// Central material.
    mat: &'a Material,
    /// Internal state of the cell.
    state: State,
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

                    if tri.plane_norm().dot(&dir).abs() > 0.1 {
                        ray = Some(Ray::new(pos, dir));
                        break;
                    }
                }
            }
        }
        let ray = ray.expect(
            "Unable to determine adequate targeting ray. Perhaps reduce angular requirement.",
        );

        let (_dist, inside, inter) = interfaces::dist_inside_inter(&ray, domain, interfaces)
            .expect("Could not determine ray interface.");
        let mat = if inside {
            inter.in_mat()
        } else {
            inter.out_mat()
        };

        Self {
            boundary,
            inter_tris,
            mat,
            state: mat.init_state().clone(),
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

    /// Reference the central material.
    pub fn mat(&self) -> &'a Material {
        self.mat
    }

    /// Reference the state.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Reference the state mutably.
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Determine the material at a given position within the cell.
    #[pre(self.boundary.contains(pos))]
    pub fn mat_at_pos(&self, pos: &Point3<f64>) -> Option<&'a Material> {
        if self.inter_tris.is_empty() {
            return Some(self.mat);
        }

        let tar = self.observation_target(pos).unwrap();
        let ray = Ray::new(*pos, Unit::new_normalize(tar - pos));
        let mut nearest: Option<(f64, &Material)> = None;
        for (inter, tris) in self.inter_tris() {
            for tri in tris {
                if let Some((dist, inside)) = tri.dist_inside(&ray) {
                    if nearest.is_none() || dist < nearest.unwrap().0 {
                        nearest = Some((
                            dist,
                            if inside {
                                inter.in_mat()
                            } else {
                                inter.out_mat()
                            },
                        ));
                    }
                }
            }
        }

        if let Some((_dist, mat)) = nearest {
            return Some(mat);
        }

        None
    }

    /// Determine the distance to the next interface along a ray's line of sight.
    #[pre(self.boundary.contains(ray.pos()))]
    pub fn inter_dist(&self, ray: &Ray) -> Option<f64> {
        let mut nearest = None;
        for (_inter, tris) in self.inter_tris() {
            for tri in tris {
                if let Some(dist) = tri.dist(ray) {
                    if nearest.is_none() || dist < nearest.unwrap() {
                        nearest = Some(dist);
                    }
                }
            }
        }

        nearest
    }

    /// Determine the distance to an interface contained within the cell, if hitting on the inside of the interface, and the normal at the intersection point.
    pub fn inter_dist_inside_norm_inter(
        &self,
        ray: &Ray,
    ) -> Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> {
        let mut nearest: Option<(f64, bool, Unit<Vector3<f64>>, &Interface)> = None;

        for (inter, tris) in &self.inter_tris {
            for tri in tris {
                if let Some((dist, inside, norm)) = tri.dist_inside_norm(ray) {
                    if nearest.is_none() || dist < nearest.unwrap().0 {
                        nearest = Some((dist, inside, norm, inter));
                    }
                }
            }
        }

        nearest
    }

    /// Select an appropriate observation target.
    #[pre(self.boundary.contains(pos))]
    fn observation_target(&self, pos: &Point3<f64>) -> Option<Point3<f64>> {
        for (_inter, tris) in self.inter_tris() {
            for tri in tris {
                let tar = tri.centre();
                if self.boundary().contains(&tar) {
                    let dir = Unit::new_normalize(tar - pos);
                    if dir.dot(tri.plane_norm()) > 1e-3 {
                        return Some(tar);
                    }
                }
            }
        }

        None
    }
}
