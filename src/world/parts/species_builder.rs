//! Species-builder functions.

use crate::{
    sci::chem::{ReactionBuilder, SpeciesBuilder},
    world::{mat::MaterialBuilder, parts::load_map},
};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Load a map of species-builders.
#[pre(dir.is_dir())]
pub fn load(
    dir: &Path,
    reactions: &HashMap<String, ReactionBuilder>,
    materials: &HashMap<String, MaterialBuilder>,
) -> HashMap<String, SpeciesBuilder> {
    let mut names = Vec::new();

    for (_name, reaction) in reactions {
        for (reactant, _s) in &reaction.reactants {
            names.push(reactant.clone());
        }
        for (product, _s) in &reaction.products {
            names.push(product.clone());
        }
    }

    for (_name, material) in materials {
        if let Some(state) = &material.state {
            if let Some(concs) = &state.concs {
                for (name, _conc) in concs {
                    names.push(name.clone());
                }
            }
            if let Some(sources) = &state.sources {
                for (name, _source) in sources {
                    names.push(name.clone());
                }
            }
        }
    }

    names.sort();
    names.dedup();

    load_map(dir, &names, "species")
}
