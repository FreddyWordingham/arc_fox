//! Reaction structure.

use super::{ProtoRate, Rate};
use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Reaction structure implementation.
#[derive(Debug)]
pub struct Reaction {
    /// List of reactant molecule indices and their associated stoichiometric coefficient of the reaction.
    reactants: Vec<(usize, i32)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(usize, i32)>,
    /// Rate of reaction.
    rate: Rate,
}

impl Reaction {
    /// Construct a new instance.
    #[pre(reactants.iter().all(|(_i, s)| *s > 0))]
    #[pre(products.iter().all(|(_i, s)| *s > 0))]
    fn new(reactants: Vec<(usize, i32)>, products: Vec<(usize, i32)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }
}

/// Proto-Reaction structure implementation.
/// Stores information required to build a reaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoReaction {
    /// List of reactant molecule indices and their associated stoichiometric coefficient of the reaction.
    reactants: Vec<(usize, String)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(usize, String)>,
    /// Rate of reaction.
    rate: ProtoRate,
}

json!(ProtoReaction);
