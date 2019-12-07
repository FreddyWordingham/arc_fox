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
        let obs = Unit::new_normalize(p - self.pos);

        let shape = img.shape();
        let h_fov_x = self.fov / 2.0;
        let h_fov_y = h_fov_x * (shape[Y as usize] as f64 / shape[X as usize] as f64);

        let phi = (self.right.dot(&obs)).asin();
        if phi.abs() > h_fov_x {
            return;
        }

        let theta = (self.up.dot(&obs)).asin();
        if theta.abs() > h_fov_y {
            return;
        }

        let dx = self.fov / shape[X as usize] as f64;

        let x = ((phi + h_fov_x) / dx).floor() as usize;
        let y = ((theta + h_fov_y) / dx).floor() as usize;

        img[(x, y)] += w;
    }
}
