//! Triangle mesh structure.

use super::{Aabb, Triangle};
use contracts::pre;

/// Triangle mesh structure.
pub struct Mesh {
    /// List of component triangles.
    tris: Vec<Triangle>,
    /// Bounding box.
    aabb: Aabb,
}

impl Mesh {
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>, aabb: Aabb) -> Self {
        let mut mins = tris[0].verts()[0];
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
