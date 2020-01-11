//! Main testing function.

use arc::{
    args,
    file::io::{map, Load},
    form,
    ord::Named,
    report,
    sci::chem::{Reaction, ReactionBuilder, Species, SpeciesBuilder},
    util::{
        dirs::init::io_dirs,
        info::exec,
        pb::Bar,
        print::term::{section, title},
    },
};
use colog;
use log::info;
use ndarray::Array1;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

form!(
    Parameters,
        num_threads: usize;
        reactions: Vec<String>;
        species: Option<Vec<String>>
);

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
    report!("Input dir", in_dir.display());

    report!("Form name", form_name.display());
    let form = Parameters::load(&form_path);
    report!(form.num_threads);

    let proto_reactions = map::<ReactionBuilder>(&in_dir.join("reactions"), &form.reactions);
    let species_list = get_species_list(&form.species.unwrap_or_else(|| vec![]), &proto_reactions);
    let proto_species = map::<SpeciesBuilder>(&in_dir.join("species"), &species_list);

    section("Building");
    let mut species = Vec::with_capacity(proto_species.len());
    for (name, proto) in proto_species {
        species.push(Species::build(name, &proto));
    }
    let mut reactions = Vec::with_capacity(proto_reactions.len());
    for (name, proto) in proto_reactions {
        reactions.push(Reaction::build(name, proto, &species));
    }

    section("Report");
    for spec in species.iter() {
        info!("Species {}", spec.name());
    }
    for reaction in reactions.iter() {
        info!("Reaction {}", reaction.name);
    }

    let mut state = State::new(
        Array1::from(vec![1.0, 1.0, 1.0, 1.0, 1.0]),
        Array1::from(vec![0.0, 0.0, 0.0, 0.0, 0.0]),
    );

    let mut file = BufWriter::new(
        File::create(out_dir.join("concentrations.csv")).expect("Unable to create output file."),
    );
    let total = 10_000;
    let mut bar = Bar::new("Counting to a million", total, 1);
    while let Some((start, end)) = bar.block(0, total / 100) {
        for conc in state.concs.iter() {
            write!(file, "{:+.6e},\t", conc).unwrap();
        }
        writeln!(file, "{}", start).unwrap();
        for _ in start..end {
            state.add_source(1.0e-3);
            state.evolve(1.0e-3, &reactions);
            bar.inc();
        }
    }
    for conc in state.concs.iter() {
        write!(file, "{:+.6e},\t", conc).unwrap();
    }
    writeln!(file, "{}", total).unwrap();
    bar.finish_with_message("Done!");

    section("Output");
    report!("Output dir", out_dir.display());
}

/// Determine the list of species involved.
fn get_species_list(
    requested: &[String],
    proto_reactions: &HashMap<String, ReactionBuilder>,
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

struct State {
    pub concs: Array1<f64>,
    pub sources: Array1<f64>,
}

impl State {
    pub fn new(concs: Array1<f64>, sources: Array1<f64>) -> Self {
        Self { concs, sources }
    }

    pub fn add_source(&mut self, dt: f64) {
        self.concs += &(&self.sources * dt);
    }

    pub fn evolve(&mut self, dt: f64, reactions: &[Reaction]) {
        let mut deltas = Array1::<f64>::zeros(self.concs.len());
        for reaction in reactions {
            let rate = reaction.rate.calc(&self.concs);

            for (index, coeff) in reaction.reactants.iter() {
                deltas[*index] += rate * coeff;
            }
            for (index, coeff) in reaction.products.iter() {
                deltas[*index] -= rate * coeff;
            }
        }

        self.concs += &(&deltas * dt);
    }
}
