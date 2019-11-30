//! Universe-Builder structure.

use crate::sci::chem::ReactionBuilder;
use serde::{Deserialize, Serialize};

/// Universe-Builder structure implementation.
/// Used to build universes.
#[derive(Debug, Deserialize, Serialize)]
pub struct UniverseBuilder {
    /// Reactions.
    reactions: Vec<ReactionBuilder>,
}

impl UniverseBuilder {
    /// Construct a new instance.
    #[inline]
    pub const fn new(reactions: Vec<ReactionBuilder>) -> Self {
        Self { reactions }
    }
}
