//! Main example function demonstrating core capabilities.

use arc::{
    args,
    file::io::Load,
    form, report,
    sci::chem::ReactionBuilder,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::mat::{InterfaceBuilder, MaterialBuilder},
};
use contracts::pre;
use log::info;
use std::path::Path;

form!(Parameters,
    reactions: Vec<String>;
    interfaces: Vec<String>
);

fn main() {
    title(&exec::name());
    colog::init();

    section("Initialisation");
    args!(
        _bin_path: String;
        form_path: String
    );
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());
    report!(
        "Loading parameters from file",
        in_dir.join(form_path).display()
    );
    let form = Parameters::load(&in_dir.join(form_path));
    let reactions = load_reactions(&in_dir.join("reactions"), &form);
    let interfaces = load_interfaces(&in_dir.join("interfaces"), &form);
    let _materials = load_materials(&in_dir.join("materials"), &interfaces);
    let _species = load_species(&in_dir.join("species"), &reactions, &interfaces);

    section("Output");
    report!("Output dir", out_dir.display());

    section("Finished");
}

#[pre(dir.is_dir())]
fn load_reactions(dir: &Path, form: &Parameters) -> Vec<ReactionBuilder> {
    form.reactions
        .iter()
        .map(|name| {
            let path = dir.join(format!("{}.json", name));
            info!("Loading reaction: {}", name);
            ReactionBuilder::load(&path)
        })
        .collect()
}

#[pre(dir.is_dir())]
fn load_interfaces(dir: &Path, form: &Parameters) -> Vec<InterfaceBuilder> {
    form.interfaces
        .iter()
        .map(|name| {
            let path = dir.join(format!("{}.json", name));
            info!("Loading interface: {}", name);
            InterfaceBuilder::load(&path)
        })
        .collect()
}

#[pre(dir.is_dir())]
fn load_materials(dir: &Path, interfaces: &[InterfaceBuilder]) -> Vec<MaterialBuilder> {
    let mut names = Vec::new();

    for interface in interfaces {
        names.push(interface.in_mat.clone());
        names.push(interface.out_mat.clone());
    }

    names.sort();
    names.dedup();

    names
        .iter()
        .map(|name| {
            let path = dir.join(format!("{}.json", name));
            info!("Loading material: {}", name);
            MaterialBuilder::load(&path)
        })
        .collect()
}

#[pre(dir.is_dir())]
fn load_species(dir: &Path, reactions: &[ReactionBuilder], interfaces: &[InterfaceBuilder]) {
    let mut names = Vec::new();

    for reaction in reactions {
        for (reactant, _s) in &reaction.reactants {
            names.push(reactant);
        }
        for (product, _s) in &reaction.products {
            names.push(product);
        }
    }

    for interface in interfaces {}

    names.sort();
    names.dedup();
}
