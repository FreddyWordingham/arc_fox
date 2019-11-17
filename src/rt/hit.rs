//! Hit enumeration.

/// Hit enumeration implementation.
#[derive(Debug)]
pub enum Hit {
    /// Scattering event.
    Scattering(f64),
    /// Cell collision.
    Cell(f64),
    /// Interface collision.
    Interface(f64),
    /// Interface collision, followed by a close cell collision.
    InterfaceCell(f64),
}
