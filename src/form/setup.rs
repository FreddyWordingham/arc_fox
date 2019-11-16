//! Setup structure.

use crate::{base::Resolution, geom::shape::ProtoMesh, json, mat::ProtoInterface};
use serde::{Deserialize, Serialize};

/// Setup structure implementation.
/// Load-time setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    /// Grid resolution.
    res: Resolution,
    /// Interfaces.
    inters: Vec<ProtoInterface>,
}

impl Setup {
    /// Construct a new instance.
    pub fn example() -> Self {
        Self {
            res: Resolution::new(9, 9, 9),
            inters: vec![ProtoInterface::new(
                ProtoMesh::new("torus".to_string(), None),
                "fog".to_string(),
                "air".to_string(),
            )],
        }
    }
}

json!(Setup);
