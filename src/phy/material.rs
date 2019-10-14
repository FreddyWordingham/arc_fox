//! Physical material structure.

/// Physical material.
/// Contains all component properties.
#[derive(Debug)]
pub struct Material {
    /// Optical properties.
    optical: OptProp,
    /// Kinetic properties.
    kinetic: KinProp,
}

impl Material {
    /// Construct a new instance.
    pub fn new(optical: OptProp, kinetic: KinProp) -> Self {
        Self { optical, kinetic }
    }

    /// Reference the optical properties.
    pub fn optical(&self) -> &OptProp {
        &self.optical
    }

    /// Reference the kinetic properties.
    pub fn kinetic(&self) -> &KinProp {
        &self.kinetic
    }
}
