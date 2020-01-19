//! Main testing binary.

use arc::{
    args,
    file::io::{map, Load},
    report,
    sci::chem::{Reaction, ReactionBuilder, Species, SpeciesBuilder, State, StateBuilder},
    util::{
        dirs::init::io_dirs,
        info::exec,
        pb::Bar,
        print::term::{section, title},
    },
};
use attr_mac::form;
use colog;
use log::info;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[form]
struct Parameters {
    num_threads: usize,
    reactions: Vec<String>,
    species: Option<Vec<String>>,
    init_state: StateBuilder,
}

pub fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    args!(_bin_path: String;
        form_name: String);

    let form_name = Path::new(&form_name);
    let (in_dir, out_dir) = io_dirs(None, None);
    let form_path = &in_dir.join(form_name);

    section("Loading");
    report!(in_dir.display(), "Input dir");

    report!(form_name.display(), "Form name");
    let form = Parameters::load(&form_path);
    report!(form.num_threads);

    let proto_reactions = map::<ReactionBuilder>(&in_dir.join("reactions"), &form.reactions);
    let species_list = get_species_list(&form.species.unwrap_or_else(|| vec![]), &proto_reactions);
    let proto_species = map::<SpeciesBuilder>(&in_dir.join("species"), &species_list);

    section("Building");
    let mut species = Vec::with_capacity(proto_species.len());
    for (name, proto) in proto_species {
        info!("Species {}", name);
        species.push(Species::build(name, &proto));
    }
    println!();
    let mut reactions = Vec::with_capacity(proto_reactions.len());
    for (name, proto) in proto_reactions {
        info!("Reaction {}", name);
        reactions.push(Reaction::build(name, proto, &species));
    }

    // let mut state = State::new(Array1::zeros(species.len()), Array1::zeros(species.len()));
    let state = State::build(form.init_state, &species);

    section("Simulation");
    let mut file = BufWriter::new(
        File::create(out_dir.join("concentrations.csv")).expect("Unable to create output file."),
    );
    let total = 100_000;
    let mut pb = Bar::new("Counting to a million", total, 1);
    while let Some((start, end)) = pb.block(0, total / 1000) {
        write!(file, "{:+.6}", start).unwrap();
        for conc in state.concs.iter() {
            write!(file, ",\t{:+.6e}", conc).unwrap();
        }
        writeln!(file).unwrap();
        for _ in start..end {
            // state.add_source(1.0e-4);
            // state.evolve(1.0e-4, &reactions);
            pb.inc();
            panic!("Removed");
        }
    }
    write!(file, "{:+.6}", total).unwrap();
    for conc in state.concs.iter() {
        write!(file, ",\t{:+.6e}", conc).unwrap();
    }
    writeln!(file).unwrap();
    pb.finish_with_message("Done!");

    section("Output");
    report!(out_dir.display(), "Output dir");
}

/// Determine the list of species involved.
fn get_species_list(
    requested: &[String],
    proto_reactions: &BTreeMap<String, ReactionBuilder>,
) -> Vec<String> {
    let mut names = (*requested).to_vec();

    for reaction in proto_reactions.values() {
        for (reactant, _) in reaction.reactants.iter() {
            names.push(reactant.to_string());
        }
        for (product, _) in reaction.products.iter() {
            names.push(product.to_string());
        }
        for catalyst in reaction.rate.catalysts() {
            names.push(catalyst.to_string());
        }
    }

    names.sort();
    names.dedup();

    names
}
