//! Reaction implementation.

use crate::{access, chem::Rate, dom::Name};
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Species reaction structure.
#[json]
pub struct Reaction {
    /// List of reactant species buy id, and their stoichiometric coefficient.
    reactants: Vec<(Name, i32)>,
    /// List of product species buy id, and their stoichiometric coefficient.
    products: Vec<(Name, i32)>,
    /// Rate.
    rate: Rate,
}

impl Reaction {
    access!(reactants, Vec<(Name, i32)>);
    access!(products, Vec<(Name, i32)>);
    access!(rate, Rate);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactants: Vec<(Name, i32)>, products: Vec<(Name, i32)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }

    /// Get a list of all species required for the reaction.
    #[inline]
    #[must_use]
    pub fn req_species(&self) -> Vec<Name> {
        let mut names = Vec::new();

        for (name, _c) in &self.reactants {
            names.push(name.clone());
        }
        for (name, _c) in &self.products {
            names.push(name.clone());
        }
        names.append(&mut self.rate.req_species());

        names.sort();
        names.dedup();

        names
    }
}

impl Display for Reaction {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some((name, coeff)) = self.reactants.first() {
            write!(fmt, "{}{}", coeff, name)?;
            for (name, coeff) in self.reactants.iter().skip(1) {
                write!(fmt, " + {}{}", coeff, name)?;
            }
        }

        write!(fmt, " -> ").expect("Could not write to formatter.");

        if let Some((name, coeff)) = self.products.first() {
            write!(fmt, "{}{}", coeff, name)?;
            for (name, coeff) in self.products.iter().skip(1) {
                write!(fmt, " + {}{}", coeff, name)?;
            }
        }

        write!(fmt, "\t{:<32} ", format!("({})", self.rate))
    }
}
