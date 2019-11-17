//! Setup structure.

use crate::{
    base::Resolution,
    chem::{ProtoRate, ProtoReaction},
    geom::{shape::ProtoMesh, ProtoTransform},
    json,
    mat::ProtoInterface,
    world::ProtoUniverse,
};
use nalgebra::{Translation3, Vector3};
use serde::{Deserialize, Serialize};

/// Setup structure implementation.
/// Load-time setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    /// Number of threads to run.
    num_threads: u64,
    /// Universe information.
    uni: ProtoUniverse,
    /// Total number of photons to run.
    total_phot: u64,
}

impl Setup {
    /// Construct a new instance.
    pub fn example() -> Self {
        Self {
            num_threads: 4,
            uni: ProtoUniverse::new(
                Resolution::new(9, 9, 9),
                Vector3::new(1.0, 1.0, 1.0),
                vec![
                    ProtoReaction::new(
                        vec![(8, "ala".to_string())],
                        vec![(1, "ppix".to_string())],
                        ProtoRate::new_first_order(0.05, "ala".to_string()),
                    ),
                    ProtoReaction::new(
                        vec![(1, "ppix".to_string()), (1, "udens".to_string())],
                        vec![(1, "death".to_string())],
                        ProtoRate::new_second_order(0.75, "ppix".to_string(), "udens".to_string()),
                    ),
                ],
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
            total_phot: 1_000,
        }
    }

    /// Get the number of threads.
    pub fn num_threads(&self) -> u64 {
        self.num_threads
    }

    /// Reference the proto-universe.
    pub fn uni(&self) -> &ProtoUniverse {
        &self.uni
    }

    /// Get the total number of photons.
    pub fn total_phot(&self) -> u64 {
        self.total_phot
    }
}

json!(Setup);
