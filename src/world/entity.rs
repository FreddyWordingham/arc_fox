//! World entity structure.

use crate::{geom::Shape, phys::Material};
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
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    #[pre(!surfs.is_empty())]
    pub fn new(surfs: Vec<Box<dyn Shape>>, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        Self {
            surfs,
            in_mat,
            out_mat,
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
}
