//! Mesh-Builder structure.

use crate::json;
use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Transform-Builder structure implementation.
/// Used to build transforms.
#[derive(Debug, Deserialize, Serialize)]
pub struct TransformBuilder {
    /// Optional translation to apply.
    trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    rot: Option<Vector3<f64>>,
    /// Optional uniform scaling to apply.
    scale: Option<f64>,
}

impl TransformBuilder {
    /// Build a transformation.
    pub fn build(self) -> Similarity3<f64> {
        let trans = self
            .trans
            .unwrap_or_else(|| Translation3::new(0.0, 0.0, 0.0));
        let rot = self.rot.unwrap_or_else(|| Vector3::new(0.0, 0.0, 0.0));
        let rot = UnitQuaternion::from_euler_angles(
            rot.x.to_radians(),
            rot.y.to_radians(),
            rot.z.to_radians(),
        );
        let scale = self.scale.unwrap_or(1.0);

        Similarity3::from_parts(trans, rot, scale)
    }
}

json!(TransformBuilder);
