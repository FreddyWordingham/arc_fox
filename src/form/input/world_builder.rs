//! World builder input form.

use super::super::proto;
use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::{env::var, path::Path};

/// Input form structure containing all information required to run the world_builder binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct WorldBuilder {
    /// Directory information.
    dir: proto::Dir,
    /// Domain information.
    dom: proto::Domain,
    /// Surface list.
    surfs: Vec<proto::Surface>,
}

impl WorldBuilder {
    /// Create an example world.
    pub fn example() -> Self {
        Self {
            dir: proto::Dir::new(
                Some(format!(
                    "{}/cwd",
                    var("ARC_DIR").expect("Environment variable ARC_DIR is not set!")
                )),
                "out".to_string(),
                "../res".to_string(),
                "mats".to_string(),
                "meshes".to_string(),
            ),
            dom: proto::Domain::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [8, 8, 8]),
            surfs: vec![proto::Surface::new(
                "prism".to_string(),
                "crown_glass".to_string(),
                "air".to_string(),
            )],
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

    /// Reference the proto-surface list.
    pub fn surfs(&self) -> &Vec<proto::Surface> {
        &self.surfs
    }

    /// Generate the material list.
    pub fn mats(&self) -> proto::Materials {
        let mut mat_list = Vec::with_capacity(self.surfs.len() * 2);

        for surf in self.surfs.iter() {
            mat_list.push(surf.inside().clone());
            mat_list.push(surf.outside().clone());
        }

        mat_list.sort();
        mat_list.dedup();

        proto::Materials::new(mat_list)
    }
}

impl Saveable for WorldBuilder {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for WorldBuilder {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
