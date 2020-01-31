//! Grid implementation.

use crate::{
    access,
    dom::{Regular, Set},
    geom::Mesh,
    uni::Interface,
};
use attr::json;
use nalgebra::Point3;

/// Grid construction form.
#[json]
pub struct Grid {
    /// Grid resolution.
    res: [usize; 3],
    /// Min point.
    mins: Point3<f64>,
    /// Max point.
    maxs: Point3<f64>,
}

impl Grid {
    access!(res, [usize; 3]);
    access!(mins, Point3<f64>);
    access!(maxs, Point3<f64>);

    pub fn form(&self, inters: &Set<Interface>, meshes: &Set<Mesh>) -> Regular {
        Regular::new(
            crate::geom::Aabb::new(self.mins, self.maxs),
            self.res,
            &inters,
            &meshes,
        )
    }
}
