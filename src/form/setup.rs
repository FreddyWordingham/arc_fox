//! Setup structure.

use crate::{
    base::Resolution,
    chem::{ProtoRate, ProtoReaction},
    dom::{ProtoGrid, ProtoRegion},
    geom::{shape::ProtoMesh, ProtoTransform},
    json, map,
    mat::ProtoInterface,
    sim::evolve::ProtoState,
    world::ProtoUniverse,
};
use nalgebra::{Translation3, Vector3};
use serde::{Deserialize, Serialize};

/// Setup structure implementation.
/// Load-time setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    /// Number of threads to run.
    num_threads: usize,
    /// Universe information.
    uni: ProtoUniverse,
    /// Total number of photons to run.
    total_phot: u64,
}

impl Setup {
    /// Construct a new instance.
    pub fn example() -> Self {
        let react_map = map!(
            "ppix_formation".to_string() =>
            ProtoReaction::new(
                vec![(8, "ala".to_string())],
                vec![(1, "ppix".to_string())],
                ProtoRate::new_first_order(0.05, "ala".to_string()),
            ),
            "cell_death_mechanism".to_string() =>
            ProtoReaction::new(
                vec![(1, "ppix".to_string()), (1, "udens".to_string())],
                vec![(1, "death".to_string())],
                ProtoRate::new_second_order(0.75, "ppix".to_string(), "udens".to_string()),
            )
        );

        let inter_map = map!(
        "top_plane".to_string() =>
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
        "torus".to_string() =>
        ProtoInterface::new(
            ProtoMesh::new("torus".to_string(), None),
            "fog".to_string(),
            "air".to_string(),
        ),
        "bottom_plane".to_string() =>
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
        ));

        let region_map = map!(
        "application_cream".to_string() =>
        ProtoRegion::new(
            ProtoMesh::new(
                "cube".to_string(),
                Some(ProtoTransform::new(
                    Some(Translation3::new(0.0, 0.0, 0.5)),
                    None,
                    Some(0.1),
                )),
            ),
            ProtoState::new(
                map!("ala".to_string() => 1.0),
                map!("ala".to_string() => 0.01),
            )
        ));

        Self {
            num_threads: 4,
            uni: ProtoUniverse::new(
                ProtoGrid::new(Resolution::new(9, 9, 9), Vector3::new(1.0, 1.0, 1.0)),
                react_map,
                inter_map,
                region_map,
            ),
            total_phot: 1_000,
        }
    }

    /// Get the number of threads.
    pub fn num_threads(&self) -> usize {
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
