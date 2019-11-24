//! Molecule-map alias.

use crate::{chem::Molecule, file::Load};
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Molecule-map alias.
pub type MolMap = HashMap<String, Molecule>;

/// Construct a molecule-map from a list of molecule ids.
#[pre(mol_dir.is_dir())]
pub fn new_mol_map(mol_dir: &Path, mut ids: Vec<String>) -> MolMap {
    info!("Constructing the molecule map...");

    ids.sort();
    ids.dedup();

    let mut mol_map = MolMap::with_capacity(ids.len());
    for id in ids.iter() {
        info!("\tLoading molecule: {}", id);
        mol_map.insert(
            id.to_string(),
            Molecule::load(&mol_dir.join(format!("{}.json", id))),
        );
    }

    info!("Loaded {} total molecules.\n", mol_map.len());

    mol_map
}
