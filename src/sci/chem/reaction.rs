//! Species reaction structure.

use crate::{access, ord::Name, sci::chem::Rate};
use attr_mac::json;

/// Species reaction structure.
#[json]
pub struct Reaction {
    /// List of reactant species buy id, and their stoichiometric coefficient.
    reactants: Vec<(Name, f64)>,
    /// List of product species buy id, and their stoichiometric coefficient.
    products: Vec<(Name, f64)>,
    /// Rate.
    rate: Rate,
}

impl Reaction {
    access!(reactants, Vec<(Name, f64)>);
    access!(products, Vec<(Name, f64)>);
    access!(rate, Rate);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactants: Vec<(Name, f64)>, products: Vec<(Name, f64)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }
}
