//! Verse implementation.

use crate::{
    access,
    chem::{Reaction, Species},
    dom::{Regular, Set},
    geom::Mesh,
    uni::{Interface, Material},
};
use std::fmt::{Display, Formatter, Result};

/// Universe instance.
pub struct Verse {
    /// Material set.
    mats: Set<Material>,
    /// Meshes set.
    meshes: Set<Mesh>,
    /// Interface set.
    inters: Set<Interface>,
    /// Species set.
    specs: Set<Species>,
    /// Reaction set.
    reacts: Set<Reaction>,
    /// Gridding.
    grid: Regular,
}

impl Verse {
    access!(mats, Set<Material>);
    access!(meshes, Set<Mesh>);
    access!(inters, Set<Interface>);
    access!(specs, Set<Species>);
    access!(reacts, Set<Reaction>);
    access!(grid, Regular);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        mats: Set<Material>,
        meshes: Set<Mesh>,
        inters: Set<Interface>,
        specs: Set<Species>,
        reacts: Set<Reaction>,
        grid: Regular,
    ) -> Self {
        Self {
            mats,
            meshes,
            inters,
            specs,
            reacts,
            grid,
        }
    }
}

impl Display for Verse {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "Materials\t({}):", self.mats.map().len())?;
        for (name, mat) in self.mats.map() {
            writeln!(fmt, "\t* {}\n\t\t{}", name, mat)?;
        }

        writeln!(fmt, "Surfaces\t({}):", self.meshes.map().len())?;
        for name in self.meshes.map().keys() {
            writeln!(fmt, "\t* {}", name)?;
        }

        writeln!(fmt, "Interfaces\t({}):", self.inters.map().len())?;
        for (name, inter) in self.inters.map() {
            writeln!(fmt, "\t* {}\n\t\t{}", name, inter)?;
        }

        writeln!(fmt, "Reactions\t({}):", self.reacts.map().len())?;
        for (name, react) in self.reacts.map() {
            writeln!(fmt, "\t* {}\n\t\t{}", name, react)?;
        }

        writeln!(fmt, "Species\t({}):", self.specs.map().len())?;
        for (name, spec) in self.specs.map() {
            writeln!(fmt, "\t* {}\n\t\t{}", name, spec)?;
        }

        write!(fmt, "grid:\t{}", self.grid)
    }
}
