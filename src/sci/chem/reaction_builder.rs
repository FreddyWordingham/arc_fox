//! Reaction-Builder structure.

use crate::{json, sci::chem::RateBuilder};
use serde::{Deserialize, Serialize};

/// Reaction-Builder structure implementation.
/// Used to build reactions.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionBuilder {
    /// List of reactant molecule indices and their associated stoichiometric coefficient of the reaction.
    reactants: Vec<(String, i32)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(String, i32)>,
    /// Rate of reaction.
    rate: RateBuilder,
}

json!(ReactionBuilder);
