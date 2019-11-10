//! Triangle mesh structure.

use super::{Aabb, Triangle};
use crate::dim::Greek::Alpha;
use contracts::pre;

/// Triangle mesh surface.
pub struct Mesh {
    /// List of component triangles.
    tris: Vec<Triangle>,
    /// Bounding box.
    aabb: Aabb,
}

impl Mesh {
    /// Construct a new instance.
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>) -> Self {
        let mut mins = tris[0].verts()[Alpha as usize];
        let mut maxs = mins;
        for t in tris.iter() {
            for v in t.verts().iter() {
                for i in 0..3 {
                    if mins[i] > v[i] {
                        mins[i] = v[i];
                    } else if maxs[i] < v[i] {
                        maxs[i] = v[i];
                    }
                }
            }
        }

        Self {
            tris,
            aabb: Aabb::new(mins, maxs),
        }
    }
}
