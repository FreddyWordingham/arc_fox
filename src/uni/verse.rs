//! Universe instancing struct.

use crate::{
    ord::Set,
    sci::chem::{Reaction, Species},
};

/// Universe instance.
pub struct Verse {
    /// Reaction set.
    reactions: Set<Reaction>,
    /// Species set.
    species: Set<Species>,
}

impl Verse {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactions: Set<Reaction>, species: Set<Species>) -> Self {
        Self { reactions, species }
    }
}
