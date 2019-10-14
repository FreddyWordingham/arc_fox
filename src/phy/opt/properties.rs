//! Optical properties structure.

/// Optical physical properties.
/// Contains parameters governing how photons interact within the material.
#[derive(Debug)]
pub struct Properties<F>
where
    F: Fn(f64) -> f64,
{
    /// Scattering coefficient [m^-1].
    scat_coeff: F,
}
