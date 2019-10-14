//! Kinetic statistical properties structure.

/// Kinetic statistical physical properties.
/// Contains parameters governing how number density changes over space with time.
#[derive(Debug)]
pub struct Properties {
    /// Diffusion coefficient [m^2/t].
    diff_coeff: f64,
}
