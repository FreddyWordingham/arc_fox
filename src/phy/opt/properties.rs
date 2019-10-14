//! Optical properties structure.

/// Optical physical properties.
/// Contains parameters governing how photons interact within the material.
#[derive(Debug)]
pub struct Properties {
    /// Scattering coefficient [m^-1].
    scat_coeff: f64,
}
