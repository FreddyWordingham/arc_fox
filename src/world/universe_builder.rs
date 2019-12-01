//! Universe-Builder structure.

use crate::{sci::chem::ReactionBuilder, world::mat::InterfaceBuilder};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Universe-Builder structure implementation.
/// Used to build universes.
#[derive(Debug, Deserialize, Serialize)]
pub struct UniverseBuilder {
    /// Reactions.
    reactions: Vec<ReactionBuilder>,
    /// Interfaces.
    interfaces: Vec<InterfaceBuilder>,
}

impl UniverseBuilder {
    /// Construct a new instance.
    #[pre(!interfaces.is_empty())]
    pub fn new(reactions: Vec<ReactionBuilder>, interfaces: Vec<InterfaceBuilder>) -> Self {
        Self {
            reactions,
            interfaces,
        }
    }
}
