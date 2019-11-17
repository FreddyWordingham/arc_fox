//! Reaction structure.

use super::{ProtoRate, Rate};
use crate::json;
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Reaction structure implementation.
#[derive(Debug)]
pub struct Reaction {
    /// List of reactant molecule indices and their associated stoichiometric coefficient of the reaction.
    reactants: Vec<(i32, usize)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(i32, usize)>,
    /// Rate of reaction.
    rate: Rate,
}

impl Reaction {
    /// Construct a new instance.
    #[pre(reactants.iter().all(|(s, _i)| *s > 0))]
    #[pre(products.iter().all(|(s, _i)| *s > 0))]
    pub fn new(reactants: Vec<(i32, usize)>, products: Vec<(i32, usize)>, rate: Rate) -> Self {
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
    reactants: Vec<(i32, String)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(i32, String)>,
    /// Rate of reaction.
    rate: ProtoRate,
}

impl ProtoReaction {
    /// Construct a new instance.
    #[pre(!reactants.is_empty())]
    #[pre(!products.is_empty())]
    #[pre(reactants.iter().all(|(s, _id)| *s > 0))]
    #[pre(products.iter().all(|(s, _id)| *s > 0))]
    pub fn new(
        reactants: Vec<(i32, String)>,
        products: Vec<(i32, String)>,
        rate: ProtoRate,
    ) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }

    /// Reference the list of reactants.
    pub fn reactants(&self) -> &Vec<(i32, String)> {
        &self.reactants
    }

    /// Reference the list of products.
    pub fn products(&self) -> &Vec<(i32, String)> {
        &self.products
    }

    /// Reference the proto-rate.
    pub fn rate(&self) -> &ProtoRate {
        &self.rate
    }
}

json!(ProtoReaction);
