//! PTFE input form.

use super::super::proto;
use crate::file::{as_json, from_json, Loadable, Saveable};
use nalgebra::{Point3, Unit, Vector3};
use serde::{Deserialize, Serialize};
use std::{env::var, path::Path};

/// Input form structure containing all information required to run the ptfe binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ptfe {
    /// Directory information.
    dir: proto::Dir,
    /// Domain information.
    dom: proto::Domain,
    /// Number of samples.
    num_phot: usize,
    /// Laser emission wavelength.
    emission_wavelength: f64,
    /// Laser emission position.
    emission_pos: [f64; 3],
    /// Laser emission direction.
    emission_dir: [f64; 3],
}

impl Ptfe {
    /// Create an example ptfe form.
    pub fn example() -> Self {
        Self {
            dir: proto::Dir::new(
                Some(format!(
                    "{}/cwd",
                    var("ARC_DIR").expect("Environment variable ARC_DIR is not set!")
                )),
                "out".to_string(),
                "../res".to_string(),
                "mats/basic".to_string(),
                "meshes/basic".to_string(),
            ),
            dom: proto::Domain::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [1, 1, 1]),
            emission_wavelength: 830e-9,
            emission_pos: [-1.0, 0.0, 0.0],
            emission_dir: [1.0, 0.0, 0.0],
            num_phot: 1_000_000,
        }
    }

    /// Reference the directory proto-structure.
    pub fn dir(&self) -> &proto::Dir {
        &self.dir
    }

    /// Reference the domain proto-structure.
    pub fn dom(&self) -> &proto::Domain {
        &self.dom
    }

    /// Get the number of photons.
    pub fn num_phot(&self) -> usize {
        self.num_phot
    }

    /// Get the laser emission wavelength.
    pub fn emission_wavelength(&self) -> f64 {
        self.emission_wavelength
    }

    /// Get the laser emission position.
    pub fn emission_pos(&self) -> Point3<f64> {
        Point3::new(
            self.emission_pos[0],
            self.emission_pos[1],
            self.emission_pos[2],
        )
    }

    /// Get the laser emission direction.
    pub fn emission_dir(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(Vector3::new(
            self.emission_dir[0],
            self.emission_dir[1],
            self.emission_dir[2],
        ))
    }
}

impl Saveable for Ptfe {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Ptfe {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
