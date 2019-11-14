//! Reaction structure.

/// Chemical reaction proto-structure.
pub struct Reaction {
    /// Reactants, their coefficients, and their order contribution.
    reactants: Vec<(String, i32, i32)>,
    /// Products and their coefficients.
    products: Vec<(String, i32)>,
    /// Rate constant.
    rate_const: f64,
}
