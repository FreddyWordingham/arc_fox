//! Reactions type alias.

use crate::sci::chem::Reaction;
use log::info;
use std::{collections::BTreeMap, path::Path};

type Reactions = BTreeMap<String, Reaction>;

pub fn load(dir: &Path, names: &[&str]) -> Reactions {
    let reactions = Reactions::new();

    for name in names {
        info!("Loading reaction: {}", name);
        let _path = dir.join(name);

        // reactions.insert(name.to_string(), Reaction::load(path));
    }

    reactions
}
