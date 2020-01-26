//! Functions for set of reactions.

use crate::ord::Set;
use crate::sci::chem::Reaction;

/// Get a list of all species required for the set of reactions.
#[inline]
#[must_use]
pub fn req_species(reactions: &Set<Reaction>) -> Vec<String> {
    let mut species = Vec::new();

    for r in reactions.map().values() {
        species.append(&mut r.req_species());
    }

    species.sort();
    species.dedup();

    species
}
