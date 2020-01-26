//! Verse form structure.

use crate::{ord::Name, uni::Verse as Universe};
use attr_mac::json;
use std::path::Path;

/// Verse loading form.
#[json]
pub struct Verse {
    /// Optional list of reactions.
    reactions: Option<Vec<Name>>,
    /// Optional list of additional species.
    species: Option<Vec<Name>>,
}

impl Verse {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactions: Option<Vec<Name>>, species: Option<Vec<Name>>) -> Self {
        Self { reactions, species }
    }

    /// Form a manifested instance.
    #[inline]
    #[must_use]
    pub fn form(&self, in_dir: &Path) -> Universe {
        Universe::load(
            in_dir,
            self.reactions.as_ref().unwrap_or(&vec![]),
            self.species.as_ref().unwrap_or(&vec![]),
        )
    }
}
