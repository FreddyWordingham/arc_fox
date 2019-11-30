//! Mesh-Builder structure.

use crate::json;
use nalgebra::{Translation3, Vector3};
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

json!(TransformBuilder);
