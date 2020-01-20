//! Triangular-mesh structure.

use crate::{
    sci::math::geom::{Aabb, SmoothTriangle},
    util::list::alphabet::Greek::Alpha,
};

/// Mesh structure implementation.
/// Forms the surface of the majority of complex components.
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
}

impl Mesh {
    /// Construct a new instance.
    pub fn new(tris: Vec<SmoothTriangle>) -> Self {
        let mut mins = tris[0].tri().verts()[Alpha as usize];
        let mut maxs = mins;

        for tri in tris.iter() {
            for v in tri.tri().verts().iter() {
                for i in 0..3 {
                    if mins[i] > v[i] {
                        mins[i] = v[i];
                    } else if maxs[i] < v[i] {
                        maxs[i] = v[i];
                    }
                }
            }
        }

        let aabb = Aabb::new(mins, maxs);

        Self { aabb, tris }
    }
}
