//! Ray structure.

use crate::sci::math::Normal;
use contracts::{post, pre};
use nalgebra::{Point3, Rotation3, Unit, Vector3};

/// Ray structure implementation.
#[derive(Debug, Clone)]
pub struct Ray {
    /// Origin position.
    pos: Point3<f64>,
    /// Facing direction.
    dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[pre((dir.is_normal()))]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }

    /// Reference the origin position.
    pub const fn pos(&self) -> &Point3<f64> {
        &self.pos
    }

    /// Set the origin position.
    pub fn set_pos(&mut self, pos: Point3<f64>) {
        self.pos = pos;
    }

    /// Reference the facing direction.
    #[post((ret.is_normal()))]
    pub fn dir(&self) -> &Unit<Vector3<f64>> {
        &self.dir
    }

    /// Set the facing direction.
    #[pre((dir.is_normal()))]
    pub fn set_dir(&mut self, dir: Unit<Vector3<f64>>) {
        self.dir = dir;
    }

    /// Move along the direction the given distance.
    #[pre(dist > 0.0)]
    pub fn travel(&mut self, dist: f64) {
        self.pos += self.dir.as_ref() * dist;
    }

    /// Pitch towards the z-axis and then roll around previous direction.
    #[post((self.dir.magnitude() - 1.0).abs() < 1.0e-6)]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let arbit_axis = if (self.dir.z.abs() - 1.0) >= 1.0e-1 {
            Vector3::z_axis()
        } else {
            Vector3::y_axis()
        };

        let pitch_axis = Unit::new_normalize(self.dir.cross(&arbit_axis));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
        self.dir.renormalize(); // TODO: Can we make this cheaper?
    }
}
