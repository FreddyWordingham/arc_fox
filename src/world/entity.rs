//! World entity structure.

use crate::{
    geom::{Aabb, Shape},
    phys::Material,
};
use contracts::pre;

/// World entity structure.
/// Binds a material to a geometry.
pub struct Entity<'a> {
    /// Surface geometry.
    surfs: Vec<Box<dyn Shape>>,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
    /// Bounding aabb.
    boundary: Aabb,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: Vec<Box<dyn Shape>>, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        let mut mins = surfs[0].boundary().mins().clone();
        let mut maxs = surfs[0].boundary().maxs().clone();

        for surf in surfs.iter().skip(1) {
            let boundary = surf.boundary();

            let surf_min = boundary.mins();
            let surf_max = boundary.maxs();

            for i in 0..3 {
                if surf_min[i] < mins[i] {
                    mins[i] = surf_min[i];
                }

                if surf_max[i] > maxs[i] {
                    maxs[i] = surf_min[i];
                }
            }
        }

        Self {
            surfs,
            in_mat,
            out_mat,
            boundary: Aabb::new(mins, maxs),
        }
    }

    /// Access the surface list.
    pub fn surfs(&self) -> &Vec<Box<dyn Shape>> {
        &self.surfs
    }

    /// Reference the inside material.
    pub fn in_mat(&self) -> &'a Material {
        &self.in_mat
    }

    /// Reference the outside material.
    pub fn out_mat(&self) -> &'a Material {
        &self.out_mat
    }

    /// Reference the boundary.
    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }
}
