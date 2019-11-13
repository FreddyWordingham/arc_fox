//! MCRT form structure.

use super::{Entity, Transform};
use crate::{index::Resolution, json};
use contracts::post;
use nalgebra::Translation3;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monte-Carlo Radiative Transfer input form parameters.
#[derive(Serialize, Deserialize)]
pub struct Mcrt {
    /// Resolution of the grid.
    res: [usize; 3],
    /// Grid extension in each direction.
    half_widths: Vector3<f64>,
    /// Number of photons to run.
    total_phot: u64,
    /// Number of threads to use.
    num_threads: usize,
    /// Entity list.
    ents: Vec<Entity>,
}

impl Mcrt {
    /// Construct an example instance.
    pub fn example() -> Self {
        Self {
            res: [21, 21, 21],
            half_widths: Vector3::new(1.5, 1.5, 2.0),
            total_phot: 1_000,
            num_threads: 1,
            ents: vec![
                Entity::new(
                    "ent-0".to_string(),
                    "air".to_string(),
                    "fog".to_string(),
                    "plane".to_string(),
                    Some(Transform::new(
                        Some(Translation3::new(0.0, 0.0, 1.0)),
                        None,
                        Some(2.0),
                    )),
                ),
                Entity::new(
                    "ent-1".to_string(),
                    "fog".to_string(),
                    "air".to_string(),
                    "torus".to_string(),
                    None,
                ),
                Entity::new(
                    "ent-2".to_string(),
                    "fog".to_string(),
                    "air".to_string(),
                    "plane".to_string(),
                    Some(Transform::new(
                        Some(Translation3::new(0.0, 0.0, -1.0)),
                        None,
                        Some(2.0),
                    )),
                ),
            ],
        }
    }

    /// Get the resolution.
    pub fn res(&self) -> Resolution {
        Resolution::new(self.res[0], self.res[1], self.res[2])
    }

    /// Reference the entity list.
    pub fn ents(&self) -> &Vec<Entity> {
        &self.ents
    }

    /// Reference the half-width values.
    #[post(ret.iter().all(|x| *x > 0.0))]
    pub fn half_widths(&self) -> &Vector3<f64> {
        &self.half_widths
    }

    /// Get the number of threads to use.
    #[post(ret > 0)]
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

json!(Mcrt);
