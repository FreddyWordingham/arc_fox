//! Domain cell structure.

use super::{EntMap, Entity};
use crate::{
    geom::{fibonnaci_ray_cast, Aabb, Collidable, Ray, Shape, Traceable},
    phys::Material,
};
use contracts::pre;
use nalgebra::{Unit, Vector3};

/// Domain cell structure.
/// Contains local spatial information.
pub struct Cell<'a> {
    /// Boundary.
    boundary: Aabb,
    /// Intersecting entity shapes.
    ents: Option<Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)>>,
    /// Default material.
    mat: &'a Material,
}

impl<'a> Cell<'a> {
    /// Construct a new instance.
    pub fn new(boundary: Aabb, ent_map: &'a EntMap<'a>) -> Self {
        let mut ents_list = Vec::new();
        for (_name, ent) in ent_map.iter() {
            if boundary.collides(ent.boundary()) {
                let mut surfs = Vec::new();
                for surf in ent.surfs() {
                    if surf.collides(&boundary) {
                        surfs.push(surf);
                    }
                }

                if !surfs.is_empty() {
                    ents_list.push((ent, surfs));
                }
            }
        }

        let ents = if ents_list.is_empty() {
            None
        } else {
            Some(ents_list)
        };

        let centre = boundary.centre();
        let n = 7;
        for i in -n..n {
            let ray = Ray::new(centre, fibonnaci_ray_cast(i, n));

            let mut nearest: Option<(f64, Unit<Vector3<f64>>, &Entity)> = None;
            for (name, ent) in ent_map {
                if ent.boundary().hit(&ray) {
                    for s in ent.surfs() {
                        if let Some((dist, norm)) = s.dist_norm(&ray) {
                            if nearest.is_none() || (dist > nearest.unwrap().0) {
                                nearest = Some((dist, norm, ent));
                            }
                        }
                    }
                }
            }

            if let Some((dist, norm, ent)) = nearest {
                if dist <= boundary.dist(&ray).unwrap() {
                    let mat = if norm.dot(&ray.dir) < 0.0 {
                        ent.out_mat()
                    } else {
                        ent.in_mat()
                    };

                    return Self {
                        boundary,
                        ents,
                        mat,
                    };
                }
            }
        }

        panic!("Unable to observe a material from a cell centre.");
    }

    /// Reference the intersecting entity shapes.
    #[pre(self.ents.is_some())]
    pub fn ents(&self) -> &Vec<(&'a Entity<'a>, Vec<&'a Box<dyn Shape>>)> {
        self.ents.as_ref().unwrap()
    }

    /// Determine if the cell contains intersecting entity surfaces.
    pub fn is_empty(&self) -> bool {
        self.ents.is_none()
    }

    /// Reference the central material.
    pub fn mat(&self) -> &Material {
        &self.mat
    }
}
