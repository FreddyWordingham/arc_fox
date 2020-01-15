//! Reaction series example function.

use arc::{
    args,
    file::{io::Load, map},
    form, report,
    sci::chem::{ReactionBuilder, SpeciesBuilder},
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

form!(Parameters, reactions: Vec<String>);

fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    let (in_dir, out_dir, param_path) = initialisation();
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());
    report!("parameters path", param_path.display());

    section("Prelude");
    let param = prelude(&param_path);
    info!("loaded parameters file");

    section("Manifest");
    let (reaction_builders, species_builders) = manifest(&in_dir, &param);
    info!("{} reactions:", reaction_builders.len());
    for name in reaction_builders.keys() {
        println!("\t{}", name);
    }
    info!("{} species:", species_builders.len());
    for name in species_builders.keys() {
        println!("\t{}", name);
    }

    section("Building");
    building();
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        param_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let param_path = &in_dir.join(param_name);

    (in_dir, out_dir, param_path.to_path_buf())
}

fn prelude(param_path: &Path) -> Parameters {
    let param = Parameters::load(&param_path);

    param
}

fn manifest(
    in_dir: &Path,
    params: &Parameters,
) -> (
    BTreeMap<String, ReactionBuilder>,
    BTreeMap<String, SpeciesBuilder>,
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

    (reaction_builders, species_builders)
}

fn building() {}
