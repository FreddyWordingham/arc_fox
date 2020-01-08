//! Species reaction structure.

use crate::sci::chem::Rate;

/// Species reaction structure.
pub struct Reaction {
    /// Unique name.
    pub name: String,
    /// List of reactant species buy index, and their stoichiometric coefficient.
    pub reactants: Vec<(usize, f64)>,
    /// List of product species buy index, and their stoichiometric coefficient.
    pub products: Vec<(usize, f64)>,
    /// Rate.
    pub rate: Rate,
}

impl Reaction {
    // Build a new instance.
    // #[inline]
    // pub const fn build(name: String, proto: ReactionBuilder, species: &[Species]) -> Self {
    //     let reactants = Vec::with_capacity(proto.reactants.len());
    //     for (name, coeff) in proto.reactants {
    //         reactants.push((, coeff));
    //     }

    //     let products = Vec::with_capacity(proto.products.len());

    //     Self { name }
    // }
}
