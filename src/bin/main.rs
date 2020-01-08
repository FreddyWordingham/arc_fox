//! Main testing function.

use arc::{
    args,
    file::io::{load_map, Load},
    form,
    ord::Named,
    report,
    sci::chem::{Reaction, ReactionBuilder, Species, SpeciesBuilder},
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use colog;
use log::info;
use std::{collections::HashMap, path::Path};

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

    let proto_reactions = load_map::<ReactionBuilder>(&in_dir.join("reactions"), &form.reactions);
    let species_list = get_species_list(&form.species.unwrap_or_else(|| vec![]), &proto_reactions);
    let proto_species = load_map::<SpeciesBuilder>(&in_dir.join("species"), &species_list);

    section("Building");
    let mut species = Vec::with_capacity(proto_species.len());
    for (name, proto) in proto_species {
        species.push(Species::build(name, proto));
    }
    let mut reactions = Vec::with_capacity(proto_reactions.len());
    for (name, proto) in proto_reactions {
        reactions.push(Reaction::build(name, proto, &species));
    }

    section("Report");
    for spec in species {
        info!("Species {}", spec.name());
    }
    for reaction in reactions {
        info!("Reaction {}", reaction.name);
    }

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
