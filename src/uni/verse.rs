//! Universe instancing struct.

use crate::{
    access,
    ord::Set,
    sci::{
        chem::{Reaction, Species},
        math::geom::shape::Mesh,
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
    /// Mesh set.
    meshes: Set<Mesh>,
}

impl Verse {
    access!(reactions, Set<Reaction>);
    access!(species, Set<Species>);
    access!(interfaces, Set<Interface>);
    access!(materials, Set<Material>);
    access!(meshes, Set<Mesh>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        reactions: Set<Reaction>,
        species: Set<Species>,
        interfaces: Set<Interface>,
        materials: Set<Material>,
        meshes: Set<Mesh>,
    ) -> Self {
        Self {
            reactions,
            species,
            interfaces,
            materials,
            meshes,
        }
    }
}

impl Display for Verse {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "Universe rundown:")?;

        writeln!(fmt, "\n{} reactions:", self.reactions.map().len())?;
        for (name, r) in self.reactions.map() {
            writeln!(fmt, "{:>16}:\t{}", name, r)?;
        }

        writeln!(fmt, "\n{} species:", self.species.map().len())?;
        for (name, s) in self.species.map() {
            writeln!(fmt, "{:>16}:\t{}", name, s)?;
        }

        writeln!(fmt, "\n{} interfaces:", self.interfaces.map().len())?;
        for (name, i) in self.interfaces.map() {
            writeln!(fmt, "{:>16}:\t{}", name, i)?;
        }

        writeln!(fmt, "\n{} materials:", self.materials.map().len())?;
        for (name, _m) in self.materials.map() {
            writeln!(fmt, "{:>16}", name)?;
        }

        writeln!(fmt, "\n{} meshes:", self.meshes.map().len())?;
        for (name, _m) in self.meshes.map() {
            writeln!(fmt, "{:>16}", name)?;
        }

        Ok(())
    }
}
