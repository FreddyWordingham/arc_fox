//! Material boundary structure.

use crate::{geom::{Triangle, Cube}, phy::Material};
use contracts::pre;

/// Material boundary structure forming the boundary between two materials.
#[derive(Debug)]
pub struct Boundary<'a> {
    /// Bounding triangles.
    tris: Vec<Triangle>,
    /// Bounding box.
    box: Cube,
    /// Inside material.
    in_mat: &'a Material,
    /// Outside material.
    out_mat: &'a Material,
}

impl<'a> Boundary<'a> {
    /// Construct a new instance.
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>, in_mat: &'a Material, out_mat: &'a Material) -> Self {
        let mut mins = tris[0].verts()[0];
        let mut maxs = mins;

        for tri in tris.iter() {
            for vert in tri.verts() {
                for d in 0..3 {
                    if vert[d] < mins[d] {
                        mins[d] = vert[d];
                    }

                    if vert[d] > maxs[d] {
                        maxs[d] = vert[d];
                    }
                }
            }
        }

        Self {
            tris,
            box: Cube::new(mins, maxs),
            in_mat,
            out_mat,
        }
    }

    /// Reference the bounding triangles.
    pub fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    /// Reference the box boundary.
    pub fn box(&self) -> &Cube {
        &self.box
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