//! Transform trait.

use crate::json;
use contracts::{post, pre};
use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Geometries implementing this trait may be transformed.
pub trait Transform: Debug {
    /// Apply the given transformation.
    fn transform(&mut self, trans: &Similarity3<f64>);
}

/// Proto-Transform structure implementation.
/// Stores information required to build a transformation.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoTransform {
    /// Optional translation to apply.
    trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    rot: Option<Vector3<f64>>,
    /// Optional uniform scaling to apply.
    scale: Option<f64>,
}

impl ProtoTransform {
    /// Construct a new instance.
    #[pre(scale.is_none() || *scale.as_ref().unwrap() > 0.0)]
    pub fn new(
        trans: Option<Translation3<f64>>,
        rot: Option<Vector3<f64>>,
        scale: Option<f64>,
    ) -> Self {
        Self { trans, rot, scale }
    }

    /// Reference the translation.
    pub fn trans(&self) -> &Option<Translation3<f64>> {
        &self.trans
    }

    /// Reference the rotation.
    pub fn rot(&self) -> &Option<Vector3<f64>> {
        &self.rot
    }

    /// Get the scaling.
    #[post(ret.is_none() || *ret.as_ref().unwrap() > 0.0)]
    pub fn scale(&self) -> Option<f64> {
        self.scale
    }

    /// Build a transformation.
    pub fn build(&self) -> Similarity3<f64> {
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

json!(ProtoTransform);
