//! Geometry transformation input.

use serde::{Deserialize, Serialize};
use nalgebra::{Isometry3, Unit, Translation3};

/// Isometry transformation setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Transformation {
    /// Translation.
    trans: [f64; 3],
    /// Roll, pitch and yaw rotation components.
    rots: [f64; 3],
}

impl Transformation {
    /// Construct a new instance.
    pub fn new(
    trans: [f64; 3],
    rots: [f64; 3],
    ) -> Self {
        Self {
            trans,
            rots
        }
    }

    /// Manifest into a completed structure.
    pub fn manifest(&self) -> Isometry3<f64> {
        let trans = Translation3::new(self.trans[0], self.trans[1], self.rots[2]);
        let rot = Unit::from_euler_angles(self.rots[0], self.rots[1], self.rots[2]);

        Isometry3::from_parts(trans, rot)
    }

    /// Reference the transformation translation.
    pub fn trans(&self) -> &[f64; 3] {
        &self.trans
    }

    /// Reference the transformation rotations.
    pub fn rots(&self) -> &[f64; 3] {
        &self.rots
    }
}
