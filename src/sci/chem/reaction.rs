//! Reaction structure.

use super::Rate;
use contracts::pre;

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
    pub fn new(reactants: Vec<(usize, i32)>, products: Vec<(usize, i32)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }

    /// Reference the reactants.
    pub fn reactants(&self) -> &Vec<(usize, i32)> {
        &self.reactants
    }

    /// Reference the products.
    pub fn products(&self) -> &Vec<(usize, i32)> {
        &self.products
    }

    /// Reference the rate.
    pub fn rate(&self) -> &Rate {
        &self.rate
    }
}
