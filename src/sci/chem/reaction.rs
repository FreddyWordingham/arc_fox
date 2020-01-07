//! Species reaction structure.

use crate::sci::chem::Rate;

/// Species reaction structure.
pub struct Reaction {
    /// Unique name.
    pub name: String,
    /// List of reactant species buy index, and their stoichiometric coefficient.
    pub reactants: Vec<(usize, f64)>,
    /// List of product species buy index, and their stoichiometric coefficient.
    pub products: Vec<(usize, f64)>,
    /// Rate.
    pub rate: Rate,
}

impl Reaction {
    /// Construct a new instance.
    #[inline]
    pub const fn new(
        name: String,
        reactants: Vec<(usize, f64)>,
        products: Vec<(usize, f64)>,
        rate: Rate,
    ) -> Self {
        Self {
            name,
            reactants,
            products,
            rate,
        }
    }
}
