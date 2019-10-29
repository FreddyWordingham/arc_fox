//! PTFE input form.

use super::super::proto;
use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::{env::var, path::Path};

/// Input form structure containing all information required to run the ptfe binary.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ptfe {
    /// Directory information.
    dir: proto::Dir,
    /// Domain information.
    dom: proto::Domain,
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
