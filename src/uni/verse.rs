//! Verse implementation.

use crate::{
    access,
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
    // /// Species set.
    // species: Set<Species>,
    // /// Reaction set.
    // reactions: Set<Reaction>,
}

impl Verse {
    access!(mats, Set<Material>);
    access!(meshes, Set<Mesh>);
    access!(inters, Set<Interface>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mats: Set<Material>, meshes: Set<Mesh>, inters: Set<Interface>) -> Self {
        Self {
            mats,
            meshes,
            inters,
        }
    }
}
