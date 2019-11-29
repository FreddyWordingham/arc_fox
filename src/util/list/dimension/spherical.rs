//! Spherical-polar dimension enumeration.

/// Spherical-polar enumeration implementation.
#[derive(Debug)]
pub enum Spherical {
    /// Radial distance.
    Rho,
    /// Polar angle.
    Theta,
    /// Azimuthal angle.
    Phi,
}
