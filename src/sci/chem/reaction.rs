//! Reaction structure.

use crate::{sci::chem::Rate, world::parts::Named};
use contracts::pre;

/// Reaction structure implementation.
#[derive(Debug)]
pub struct Reaction {
    /// Name of the reaction.
    name: String,
    /// List of reactant molecule indices and their associated stoichiometric coefficient of the reaction.
    reactants: Vec<(usize, i32)>,
    /// List of product molecule indices and their associated stoichiometric coefficient of the reaction.
    products: Vec<(usize, i32)>,
    /// Rate of reaction.
    rate: Rate,
}

impl Reaction {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    #[pre(reactants.iter().all(|(_i, s)| *s > 0))]
    #[pre(products.iter().all(|(_i, s)| *s > 0))]
    pub fn new(
        name: String,
        reactants: Vec<(usize, i32)>,
        products: Vec<(usize, i32)>,
        rate: Rate,
    ) -> Self {
        Self {
            name,
            reactants,
            products,
            rate,
        }
    }

    /// Reference the reactants.
    pub const fn reactants(&self) -> &Vec<(usize, i32)> {
        &self.reactants
    }

    /// Reference the products.
    pub const fn products(&self) -> &Vec<(usize, i32)> {
        &self.products
    }

    /// Reference the rate.
    pub const fn rate(&self) -> &Rate {
        &self.rate
    }
}

impl Named for Reaction {
    /// Reference the name.
    fn name(&self) -> &str {
        &self.name
    }
}
