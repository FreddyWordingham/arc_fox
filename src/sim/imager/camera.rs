//! Camera structure.

use crate::sci::math::Normal;
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array2;

/// Camera structure implementation.
/// Images photons.
pub struct Camera {
    /// Viewing position.
    pos: Point3<f64>,
    /// Viewing direction.
    dir: Unit<Vector3<f64>>,
    /// Field of view.
    fov: f64,
    /// Image data.
    img: Array2<f64>,
}

impl Camera {
    /// Construct a new instance.
    #[pre(dir.is_normal())]
    #[pre(fov > 0.0)]
    #[pre(res.iter().all(|s| *s > 0))]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>, fov: f64, res: [usize; 2]) -> Self {
        Self {
            pos,
            dir,
            fov,
            img: Array2::zeros(res),
        }
    }
}
