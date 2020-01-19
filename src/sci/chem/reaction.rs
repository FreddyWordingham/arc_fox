//! Species reaction structure.

use crate::{
    access,
    ord::Set,
    sci::chem::{Rate, ReactionBuilder, Species},
};

/// Species reaction structure.
pub struct Reaction {
    /// Unique name.
    name: String,
    /// List of reactant species buy index, and their stoichiometric coefficient.
    reactants: Vec<(usize, f64)>,
    /// List of product species buy index, and their stoichiometric coefficient.
    products: Vec<(usize, f64)>,
    /// Rate.
    rate: Rate,
}

impl Reaction {
    access!(name, String);
    access!(reactants, Vec<(usize, f64)>);
    access!(products, Vec<(usize, f64)>);
    access!(rate, Rate);

    /// Build a new instance.
    #[inline]
    #[must_use]
    pub fn build(name: String, proto: ReactionBuilder, species: &[Species]) -> Self {
        let mut reactants = Vec::with_capacity(proto.reactants().len());
        for (name, coeff) in proto.reactants() {
            reactants.push((
                species
                    .index_of(name)
                    .expect("Could not find reactant species in loaded list."),
                *coeff,
            ));
        }

        let mut products = Vec::with_capacity(proto.products().len());
        for (name, coeff) in proto.products() {
            products.push((
                species
                    .index_of(name)
                    .expect("Could not find product species in loaded list."),
                *coeff,
            ));
        }

        let rate = Rate::build(proto.rate().clone(), species);

        Self {
            name,
            reactants,
            products,
            rate,
        }
    }
}
