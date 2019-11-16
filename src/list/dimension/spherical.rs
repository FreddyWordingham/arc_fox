//! Spherical enumeration.

/// Spherical enumeration implementation.
#[derive(Debug)]
pub enum Spherical {
    /// Radial distance.
    Rho,
    /// Polar angle.
    Theta,
    /// Azimuthal angle.
    Phi,
}
