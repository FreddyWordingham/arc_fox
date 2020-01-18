//! Reaction builder structure.

use crate::sci::chem::RateBuilder;
use proc_mac::Json;
use serde::{Deserialize, Serialize};

/// Reaction builder structure.
#[derive(Debug, Deserialize, Serialize, Json)]
pub struct ReactionBuilder {
    /// List of reactant species buy name, and their stoichiometric coefficient.
    pub reactants: Vec<(String, f64)>,
    /// List of product species buy name, and their stoichiometric coefficient.
    pub products: Vec<(String, f64)>,
    /// Rate.
    pub rate: RateBuilder,
}
