//! Reaction builder structure.

use crate::{access, sci::chem::RateBuilder};
use attr_mac::json;

/// Reaction builder structure.
#[json]
pub struct ReactionBuilder {
    /// List of reactant species buy name, and their stoichiometric coefficient.
    reactants: Vec<(String, f64)>,
    /// List of product species buy name, and their stoichiometric coefficient.
    products: Vec<(String, f64)>,
    /// Rate.
    rate: RateBuilder,
}

impl ReactionBuilder {
    access!(reactants, Vec<(String, f64)>);
    access!(products, Vec<(String, f64)>);
    access!(rate, RateBuilder);
}
