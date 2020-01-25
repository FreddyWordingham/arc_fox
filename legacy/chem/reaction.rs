//! Species reaction structure.

use crate::{
    access,
    ord::Set,
    sci::chem::{Rate, ReactionBuilder, Species},
};

/// Species reaction structure.
pub struct Reaction {
    /// List of reactant species buy id, and their stoichiometric coefficient.
    reactants: Vec<(String, f64)>,
    /// List of product species buy id, and their stoichiometric coefficient.
    products: Vec<(String, f64)>,
    /// Rate.
    rate: Rate,
}

impl Reaction {
    access!(reactants, Vec<(String, f64)>);
    access!(products, Vec<(String, f64)>);
    access!(rate, Rate);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactants: Vec<(String, f64)>, products: Vec<(String, f64)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }
}
