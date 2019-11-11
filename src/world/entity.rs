//! World entity structure.

use super::{Identity, Material};
use crate::{
    geom::{Aabb, Mesh, Triangle},
    rt::{Ray, Traceable},
};
use contracts::{post, pre};
use log::warn;
use nalgebra::{Point3, Unit, Vector3};

/// World entity structure.
/// Binds a material to a shape.
#[derive(Debug)]
pub struct Entity<'a> {
    /// Id string.
    id: String,
    /// Surface mesh.
    mesh: Mesh,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    #[pre(!id.is_empty())]
    pub fn new(id: String, mesh: Mesh, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            id,
            mesh,
            in_mat,
            out_mat,
        }
    }

    /// Reference the surface mesh.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &Material {
        &self.in_mat
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &Material {
        &self.out_mat
    }
}

impl<'a> Identity for Entity<'a> {
    #[post(!ret.is_empty())]
    fn id(&self) -> &str {
        &self.id
    }
}

#[pre(dom.contains(&p))]
#[pre(!ents.is_empty())]
pub fn mat_at_pos_from_list<'a>(p: Point3<f64>, dom: &Aabb, ents: &'a Vec<Entity>) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new_fibonacci_spiral(p, i, n.pow(power));

            let mut nearest: Option<(f64, bool, &Entity)> = None;
            for ent in ents.iter() {
                if let Some((dist, inside)) = ent.mesh().dist_inside(&ray) {
                    if nearest.is_none() || dist < nearest.unwrap().0 {
                        nearest = Some((dist, inside, ent));
                    }
                }
            }

            if let Some((dist, inside, ent)) = nearest {
                if dist
                    <= dom
                        .dist(&ray)
                        .expect("Failed to determine internal dom distance.")
                {
                    return if inside { ent.in_mat() } else { ent.out_mat() };
                }
            }
        }

        if power < 4 {
            warn!(
                "Increasing ray-casting power to {} ({} rays)",
                power,
                (2 * n.pow(power)) + 1
            );

            power += 1;
        } else {
            break;
        }
    }

    panic!(
        "Unable to observe a material from given point after {} samples.",
        (2 * n.pow(power)) + 1
    );
}

#[pre(dom.contains(&p))]
#[pre(!ents.is_empty())]
#[pre(aabb.contains(&p))]
#[pre(!ent_tris.is_empty())]
pub fn mat_at_pos_from_sublist<'a>(
    p: Point3<f64>,
    dom: &Aabb,
    ents: &'a Vec<Entity>,
    aabb: &Aabb,
    ent_tris: &Vec<(&'a Entity, Vec<&Triangle>)>,
) -> &'a Material {
    let n: i32 = 7;
    let mut power = 2;
    loop {
        for i in -n.pow(power)..=n.pow(power) {
            let ray = Ray::new_fibonacci_spiral(p, i, n.pow(power));

            let mut nearest: Option<(f64, bool, &Entity)> = None;
            for (ent, tris) in ent_tris.iter() {
                for tri in tris.iter() {
                    if let Some((dist, inside)) = tri.dist_inside(&ray) {
                        if nearest.is_none() || dist < nearest.unwrap().0 {
                            nearest = Some((dist, inside, ent));
                        }
                    }
                }
            }

            if let Some((dist, inside, ent)) = nearest {
                if dist
                    <= aabb
                        .dist(&ray)
                        .expect("Failed to determine internal aabb distance.")
                {
                    return if inside { ent.in_mat() } else { ent.out_mat() };
                }
            }
        }

        if power < 4 {
            warn!(
                "Increasing ray-casting power to {} ({} rays)",
                power,
                (2 * n.pow(power)) + 1
            );

            power += 1;
        } else {
            break;
        }
    }

    warn!("Falling back on world-cast.");
    mat_at_pos_from_list(p, dom, ents)
}
