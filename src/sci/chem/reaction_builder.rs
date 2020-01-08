//! Reaction builder structure.

use crate::{rw_json, sci::chem::RateBuilder};
use serde::{Deserialize, Serialize};

/// Species reaction builder structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionBuilder {
    /// List of reactant species buy name, and their stoichiometric coefficient.
    pub reactants: Vec<(String, f64)>,
    /// List of product species buy name, and their stoichiometric coefficient.
    pub products: Vec<(String, f64)>,
    /// Rate.
    pub rate: RateBuilder,
}

rw_json!(ReactionBuilder);
