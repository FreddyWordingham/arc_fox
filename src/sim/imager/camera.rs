//! Camera structure.

use crate::{
    sci::math::Normal,
    util::list::dimension::Cartesian::{X, Y},
};
use contracts::pre;
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array2;

/// Camera structure implementation.
/// Images photons.
pub struct Camera {
    /// Viewing position.
    pos: Point3<f64>,
    /// Forward (viewing) direction.
    dir: Unit<Vector3<f64>>,
    /// Right direction.
    right: Unit<Vector3<f64>>,
    /// Up direction.
    up: Unit<Vector3<f64>>,
    /// Field of view.
    fov: f64,
}

impl Camera {
    /// Construct a new instance.
    #[pre(dir.is_normal())]
    #[pre(fov > 0.0)]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>, fov: f64) -> Self {
        let right = Unit::new_normalize(dir.cross(&Vector3::z_axis()));
        let up = Unit::new_normalize(dir.cross(&right));

        Self {
            pos,
            dir,
            right,
            up,
            fov,
        }
    }

    #[pre(!img.is_empty())]
    #[pre(w > 0.0)]
    #[pre(w <= 1.0)]
    pub fn observe(&self, img: &mut Array2<f64>, p: &Point3<f64>, w: f64) {
        let g = Unit::new_normalize(p - self.pos);
        let phi = (self.right.dot(&g)).asin();
        let theta = (self.up.dot(&g)).asin();

        let shape = img.shape();

        let dx = self.fov / shape[X as usize] as f64;

        let x = ((phi / dx).floor() as usize) + (shape[X as usize] / 2);
        let y = ((theta / dx).floor() as usize) + (shape[Y as usize] / 2);

        if x >= shape[X as usize] || y >= shape[Y as usize] {
            return;
        }

        img[(x, y)] += w;
    }
}
