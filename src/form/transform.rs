//! Setup form structure.

use crate::json;
use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Transform input form.
#[derive(Serialize, Deserialize)]
pub struct Transform {
    /// Optional translation to apply.
    pub trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    pub rot: Option<Vector3<f64>>,
    /// Optional uniform scaling to apply.
    pub scale: Option<f64>,
}

impl Transform {
    /// Construct a new instance.
    pub fn new(
        trans: Option<Translation3<f64>>,
        rot: Option<Vector3<f64>>,
        scale: Option<f64>,
    ) -> Self {
        Self { trans, rot, scale }
    }

    /// Construct an affine transformation.
    pub fn manifest(self) -> Similarity3<f64> {
        let trans = if let Some(trans) = self.trans {
            trans
        } else {
            Translation3::new(0.0, 0.0, 0.0)
        };

        let rot = if let Some(rot) = self.rot {
            UnitQuaternion::from_euler_angles(
                rot.x.to_radians(),
                rot.y.to_radians(),
                rot.z.to_radians(),
            )
        } else {
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0)
        };

        let scale = if let Some(scale) = self.scale {
            scale
        } else {
            1.0
        };

        Similarity3::from_parts(trans, rot, scale)
    }
}

json!(Transform);
