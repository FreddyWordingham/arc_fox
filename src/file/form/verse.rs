//! Verse form structure.

use crate::{
    ord::{
        set::{interfaces, reactions},
        Name, Set,
    },
    sci::{
        chem::{Reaction, Species},
        phys::{Interface, Material},
    },
    uni::Verse as Universe,
};
use attr_mac::json;
use std::path::Path;

/// Verse loading form.
#[json]
pub struct Verse {
    /// Optional list of reactions.
    reactions: Option<Vec<Name>>,
    /// Optional list of additional species.
    species: Option<Vec<Name>>,
    /// List of interfaces.
    interfaces: Vec<Name>,
    /// Optional list of additional materials.
    materials: Option<Vec<Name>>,
}

impl Verse {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        reactions: Option<Vec<Name>>,
        species: Option<Vec<Name>>,
        interfaces: Vec<Name>,
        materials: Option<Vec<Name>>,
    ) -> Self {
        Self {
            reactions,
            species,
            interfaces,
            materials,
        }
    }

    /// Form a manifested instance.
    #[inline]
    #[must_use]
    pub fn form(&self, in_dir: &Path) -> Universe {
        let reactions = Set::<Reaction>::load(
            &in_dir.join("reactions"),
            self.reactions.as_ref().unwrap_or(&vec![]),
            "json",
        );

        let species_names = {
            let mut rs = reactions::req_species(&reactions);
            rs.extend_from_slice(self.species.as_ref().unwrap_or(&vec![]));
            rs
        };
        let species = Set::<Species>::load(&in_dir.join("species"), &species_names, "json");

        let interfaces =
            Set::<Interface>::load(&in_dir.join("interfaces"), &self.interfaces, "json");

        let material_names = {
            let mut rm = interfaces::req_materials(&interfaces);
            rm.extend_from_slice(self.materials.as_ref().unwrap_or(&vec![]));
            rm
        };
        let materials = Set::<Material>::load(&in_dir.join("materials"), &material_names, "json");

        Universe::new(reactions, species, interfaces, materials)
    }
}
