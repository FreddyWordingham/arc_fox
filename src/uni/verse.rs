//! Universe instancing struct.

use crate::{
    ord::{set::req_species, Name, Set},
    sci::chem::{Reaction, Species},
};
use std::path::Path;

/// Universe instance.
pub struct Verse {
    /// Reaction set.
    reactions: Set<Reaction>,
}

impl Verse {
    /// Load a new instance.
    #[inline]
    #[must_use]
    pub fn load(in_dir: &Path, reaction_names: &[Name], species_names: &[Name]) -> Self {
        let reactions = Set::<Reaction>::load(&in_dir.join("reactions"), reaction_names, "json");

        let species_names = {
            let mut rs = req_species(&reactions);
            rs.extend_from_slice(species_names);
            rs
        };
        let _species = Set::<Species>::load(&in_dir.join("species"), &species_names, "json");

        Self { reactions }
    }
}
