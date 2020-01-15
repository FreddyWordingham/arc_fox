//! Reaction series example function.

use arc::{
    args,
    file::{io::Load, map},
    form, report,
    sci::chem::{Reaction, ReactionBuilder, Species, SpeciesBuilder, State, StateBuilder},
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use colog;
use log::info;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
// use std::{
// collections::BTreeMap,
// fs::File,
// io::{BufWriter, Write},
// path::Path,
// };

form!(
    Parameters,
        reactions: Vec<String>;
        init_state: StateBuilder
);

fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());
    report!("parameters path", params_path.display());

    section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    section("Manifest");
    let (reaction_builders, species_builders, state_builder) = manifest(params, &in_dir);
    info!("found {} reactions:", reaction_builders.len());
    for name in reaction_builders.keys() {
        info!("\t{}", name);
    }
    info!("found {} species:", species_builders.len());
    for name in species_builders.keys() {
        info!("\t{}", name);
    }

    section("Building");
    let (species, reactions, state) = building(reaction_builders, species_builders, state_builder);
    info!("built {} species:", species.len());
    for (spec, (conc, source)) in species
        .iter()
        .zip(state.concs.iter().zip(state.sources.iter()))
    {
        info!("\t{}\t{}\t{}", spec.name, conc, source);
    }
    info!("built {} reactions:", reactions.len());
    for react in reactions {
        info!("\t{}", react.name);
    }
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn prelude(params_path: &Path) -> Parameters {
    let params = Parameters::load(&params_path);

    params
}

fn manifest(
    params: Parameters,
    in_dir: &Path,
) -> (
    BTreeMap<String, ReactionBuilder>,
    BTreeMap<String, SpeciesBuilder>,
    StateBuilder,
) {
    let reaction_builders = map::<ReactionBuilder>(&in_dir.join("reactions"), &params.reactions);

    let mut species_names = Vec::new();
    for builder in reaction_builders.values() {
        for (reactant, _) in builder.reactants.iter() {
            species_names.push(reactant.to_string());
        }
        for (product, _) in builder.products.iter() {
            species_names.push(product.to_string());
        }
        for catalyst in builder.rate.catalysts() {
            species_names.push(catalyst.to_string());
        }
    }
    species_names.sort();
    species_names.dedup();
    let species_builders = map::<SpeciesBuilder>(&in_dir.join("species"), &species_names);

    let state_builder = params.init_state;

    (reaction_builders, species_builders, state_builder)
}

fn building(
    reaction_builders: BTreeMap<String, ReactionBuilder>,
    species_builders: BTreeMap<String, SpeciesBuilder>,
    state_builder: StateBuilder,
) -> (Vec<Species>, Vec<Reaction>, State) {
    let mut species = Vec::with_capacity(species_builders.len());
    for (name, builder) in species_builders {
        species.push(Species::build(name, &builder));
    }

    let mut reactions = Vec::with_capacity(reaction_builders.len());
    for (name, builder) in reaction_builders {
        reactions.push(Reaction::build(name, builder, &species));
    }

    let state = State::build(state_builder, &species);

    (species, reactions, state)
}
