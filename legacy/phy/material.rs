//! Physical material structure.

use super::{kin::Properties as KinProp, opt::Properties as OptProp};
use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Physical material.
/// Contains all component properties.
#[derive(Debug, Deserialize, Serialize)]
pub struct Material {
    /// Optical properties.
    opt: Option<OptProp>,
    /// Kinetic properties.
    kin: Option<KinProp>,
}

impl Material {
    /// Construct a new instance.
    pub fn new(opt: Option<OptProp>, kin: Option<KinProp>) -> Self {
        Self { opt, kin }
    }

    /// Reference the optical properties.
    #[pre(self.opt.is_some())]
    pub fn opt(&self) -> &OptProp {
        match self.opt {
            Some(ref o) => o,
            None => unreachable!(
                "Attempting to access optics of a material that doesn't interact with photons."
            ),
        }
    }

    /// Reference the kinetic properties.
    #[pre(self.kin.is_some())]
    pub fn kin(&self) -> &KinProp {
        match self.kin {
            Some(ref k) => k,
            None => {
                unreachable!("Attempting to access kinetics of a material that doesn't diffuse.")
            }
        }
    }
}

impl Saveable for Material {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Material {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
