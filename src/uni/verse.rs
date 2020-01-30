//! Verse implementation.

use crate::{
    access,
    chem::{Reaction, Species},
    dom::Set,
    geom::Mesh,
    uni::{Interface, Material},
};

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
}

impl Verse {
    access!(mats, Set<Material>);
    access!(meshes, Set<Mesh>);
    access!(inters, Set<Interface>);
    access!(specs, Set<Species>);
    access!(reacts, Set<Reaction>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        mats: Set<Material>,
        meshes: Set<Mesh>,
        inters: Set<Interface>,
        specs: Set<Species>,
        reacts: Set<Reaction>,
    ) -> Self {
        Self {
            mats,
            meshes,
            inters,
            specs,
            reacts,
        }
    }
}
