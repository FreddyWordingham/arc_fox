//! Proto-Reaction structure.

use contracts::pre;

/// Proto-Reaction structure.
/// Used to manifest chem reactions.
pub struct Reaction {
    /// List of reactant keyname-ratio pairs.
    reactants: Vec<(i32, &'static str)>,
    /// List of product keyname-ratio pairs.
    products: Vec<(i32, &'static str)>,
    /// Rate of reaction.
    rate: f64,
}

impl Reaction {
    #[pre(!reactants.is_empty())]
    #[pre(!products.is_empty())]
    #[pre(rate > 0.0)]
    pub fn new(
        reactants: Vec<(i32, &'static str)>,
        products: Vec<(i32, &'static str)>,
        rate: f64,
    ) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }
}
