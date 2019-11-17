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
use std::collections::HashMap;

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
        let mut react_map = HashMap::with_capacity(2);
        react_map.insert(
            "ppix_formation".to_string(),
            ProtoReaction::new(
                vec![(8, "ala".to_string())],
                vec![(1, "ppix".to_string())],
                ProtoRate::new_first_order(0.05, "ala".to_string()),
            ),
        );
        react_map.insert(
            "cell_death_mechanism".to_string(),
            ProtoReaction::new(
                vec![(1, "ppix".to_string()), (1, "udens".to_string())],
                vec![(1, "death".to_string())],
                ProtoRate::new_second_order(0.75, "ppix".to_string(), "udens".to_string()),
            ),
        );

        let mut inter_map = HashMap::with_capacity(3);
        inter_map.insert(
            "top_plane".to_string(),
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
        );
        inter_map.insert(
            "torus".to_string(),
            ProtoInterface::new(
                ProtoMesh::new("torus".to_string(), None),
                "fog".to_string(),
                "air".to_string(),
            ),
        );
        inter_map.insert(
            "bottom_plane".to_string(),
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
        );

        Self {
            num_threads: 4,
            uni: ProtoUniverse::new(
                Resolution::new(9, 9, 9),
                Vector3::new(1.0, 1.0, 1.0),
                react_map,
                inter_map,
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
