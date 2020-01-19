//! Reaction builder structure.

use crate::sci::chem::RateBuilder;
use attr_mac::json;

/// Reaction builder structure.
#[json]
pub struct ReactionBuilder {
    /// List of reactant species buy name, and their stoichiometric coefficient.
    pub reactants: Vec<(String, f64)>,
    /// List of product species buy name, and their stoichiometric coefficient.
    pub products: Vec<(String, f64)>,
    /// Rate.
    pub rate: RateBuilder,
}
