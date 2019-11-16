//! Setup structure.

use crate::{
    base::Resolution,
    geom::{shape::ProtoMesh, ProtoTransform},
    json,
    mat::ProtoInterface,
    world::ProtoUniverse,
};
use nalgebra::Translation3;
use serde::{Deserialize, Serialize};

/// Setup structure implementation.
/// Load-time setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    /// Universe information.
    uni: ProtoUniverse,
}

impl Setup {
    /// Construct a new instance.
    pub fn example() -> Self {
        Self {
            uni: ProtoUniverse::new(
                Resolution::new(9, 9, 9),
                vec![
                    ProtoInterface::new(
                        ProtoMesh::new(
                            "plane".to_string(),
                            Some(ProtoTransform::new(
                                Some(Translation3::new(0.0, 0.0, 0.75)),
                                None,
                                Some(1.05),
                            )),
                        ),
                        "fog".to_string(),
                        "air".to_string(),
                    ),
                    ProtoInterface::new(
                        ProtoMesh::new("torus".to_string(), None),
                        "fog".to_string(),
                        "air".to_string(),
                    ),
                    ProtoInterface::new(
                        ProtoMesh::new(
                            "plane".to_string(),
                            Some(ProtoTransform::new(
                                Some(Translation3::new(0.0, 0.0, -0.75)),
                                None,
                                Some(1.05),
                            )),
                        ),
                        "air".to_string(),
                        "fog".to_string(),
                    ),
                ],
            ),
        }
    }
}

json!(Setup);
