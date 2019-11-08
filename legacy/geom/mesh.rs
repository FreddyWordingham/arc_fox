//! Mesh structure.

use super::{Aabb, Triangle};
use contracts::pre;

/// Mesh structure.
/// Composed of triangles.
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// Triangle list.
    tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new instance.
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>) -> Self {
        let mut mins = tris[0].verts()[0];
        let mut maxs = mins;
        for t in tris.iter() {
            for v in t.verts() {
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
            aabb: Aabb::new(mins, maxs),
            tris,
        }
    }
}
