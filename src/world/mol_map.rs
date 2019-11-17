//! Molecule-map alias.

use crate::{chem::Molecule, file::Load};
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

/// Molecule-map alias.
pub type MolMap = HashMap<String, Molecule>;

/// Construct a molecule-map from a list of molecule names.
#[pre(mol_dir.is_dir())]
#[pre(!names.is_empty())]
#[post(!ret.is_empty())]
pub fn new_mol_map(mol_dir: &Path, mut names: Vec<String>) -> MolMap {
    info!("Constructing the molecule map...");

    names.sort();
    names.dedup();

    let mut mol_map = MolMap::with_capacity(names.len());
    for name in names.iter() {
        info!("Loading molecule: {}", name);
        mol_map.insert(
            name.to_string(),
            Molecule::load(&mol_dir.join(format!("{}.json", name))),
        );
    }

    info!("Loaded {} total molecule.", mol_map.len());

    mol_map
}
