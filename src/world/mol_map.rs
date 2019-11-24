//! Molecule-map alias.

use crate::{chem::Molecule, file::Load, util::progress::bar};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Molecule-map alias.
pub type MolMap = HashMap<String, Molecule>;

/// Construct a molecule-map from a list of molecule ids.
#[pre(mol_dir.is_dir())]
pub fn new_mol_map(mol_dir: &Path, mut ids: Vec<String>) -> MolMap {
    let pb = bar("Constructing molecules", ids.len() as u64);

    ids.sort();
    ids.dedup();

    let mut mol_map = MolMap::with_capacity(ids.len());
    for id in ids.iter() {
        pb.inc(1);

        mol_map.insert(
            id.to_string(),
            Molecule::load(&mol_dir.join(format!("{}.json", id))),
        );
    }

    pb.finish_with_message("Molecules constructed.");

    mol_map
}
