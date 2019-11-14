//! Reaction structure.

use crate::{json, world::Identity};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Chemical reaction structure.
#[derive(Serialize, Deserialize)]
pub struct Reaction {
    /// Identification string.
    id: String,
    /// Rate.
    rate: f64,
    /// Reactants and their relative ratios.
    reactants: Vec<(String, i32)>,
    /// Products and their relative ratios.
    products: Vec<(String, i32)>,
}

impl Reaction {
    #[pre(!id.is_empty())]
    #[pre(rate > 0.0)]
    #[pre(!reactants.is_empty())]
    #[pre(reactants.iter().all(|(_id, n)| *n > 0))]
    #[pre(!products.is_empty())]
    #[pre(products.iter().all(|(_id, n)| *n > 0))]
    /// Construct a new instance.
    pub fn new(
        id: String,
        rate: f64,
        reactants: Vec<(String, i32)>,
        products: Vec<(String, i32)>,
    ) -> Self {
        Self {
            id,
            rate,
            products,
            reactants,
        }
    }
}

impl Identity for Reaction {
    fn id(&self) -> &str {
        &self.id
    }
}

json!(Reaction);
