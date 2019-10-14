//! Physical material structure.

use super::kin::Properties as KinProp;
use super::opt::Properties as OptProp;
use contracts::pre;

/// Physical material.
/// Contains all component properties.
#[derive(Debug)]
pub struct Material {
    /// Optical properties.
    optical: Option<OptProp>,
    /// Kinetic properties.
    kinetic: Option<KinProp>,
}

impl Material {
    /// Construct a new instance.
    pub fn new(optical: Option<OptProp>, kinetic: Option<KinProp>) -> Self {
        Self { optical, kinetic }
    }

    /// Reference the optical properties.
    #[pre(self.optical.is_some())]
    pub fn optical(&self) -> &OptProp {
        match self.optical {
            Some(ref o) => o,
            None => unreachable!(
                "Attempting to access optics of a material that doesn't interact with photons."
            ),
        }
    }

    /// Reference the kinetic properties.
    #[pre(self.kinetic.is_some())]
    pub fn kinetic(&self) -> &KinProp {
        match self.kinetic {
            Some(ref k) => k,
            None => {
                unreachable!("Attempting to access kinetics of a material that doesn't diffuse.")
            }
        }
    }
}
