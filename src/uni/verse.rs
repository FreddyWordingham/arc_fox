//! Universe instancing struct.

use crate::{
    access,
    ord::Set,
    sci::{
        chem::{Reaction, Species},
        phys::{Interface, Material},
    },
};
use std::fmt::{Display, Formatter, Result};

/// Universe instance.
pub struct Verse {
    /// Reaction set.
    reactions: Set<Reaction>,
    /// Species set.
    species: Set<Species>,
    /// Interface set.
    interfaces: Set<Interface>,
    /// Material set.
    materials: Set<Material>,
}

impl Verse {
    access!(reactions, Set<Reaction>);
    access!(species, Set<Species>);
    access!(interfaces, Set<Interface>);
    access!(materials, Set<Material>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        reactions: Set<Reaction>,
        species: Set<Species>,
        interfaces: Set<Interface>,
        materials: Set<Material>,
    ) -> Self {
        Self {
            reactions,
            species,
            interfaces,
            materials,
        }
    }
}

impl Display for Verse {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "Universe rundown:")?;

        writeln!(fmt, "{} known reactions:", self.reactions.map().len())?;
        for (name, r) in self.reactions.map() {
            writeln!(fmt, "{:>32}:\t{}", name, r)?;
        }

        Ok(())
    }
}
