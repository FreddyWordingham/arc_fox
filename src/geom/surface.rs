use nalgebra::{Point3, Unit, Vector3};

pub enum Surface {
    Plane {
        /// Position on the plane.
        pos: Point3<f64>,
        /// Normal.
        norm: Unit<Vector3<f64>>,
    },
}

impl Surface {
    /// Construct a new Plane instance.
    pub fn new_plane(pos: Point3<f64>, norm: Unit<Vector3<f64>>) -> Self {
        Self::Plane { pos, norm }
    }
}
