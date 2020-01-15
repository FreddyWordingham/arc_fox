//! Reaction series example function.

use arc::{
    args,
    file::{io::Load, map},
    form, report,
    sci::chem::ReactionBuilder,
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

    let (in_dir, out_dir, param_path) = initialisation();
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());
    report!("parameters path", param_path.display());

    let param = prelude(&param_path);

    let reaction_builders = manifest(&in_dir, &param);
    info!("{} reactions:", reaction_builders.len());
    for name in reaction_builders.keys() {
        println!("\t{}", name);
    }

    building();
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    section("Initialisation");

    args!(_bin_path: String;
        param_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let param_path = &in_dir.join(param_name);

    (in_dir, out_dir, param_path.to_path_buf())
}

fn prelude(param_path: &Path) -> Parameters {
    section("Prelude");

    let param = Parameters::load(&param_path);

    param
}

fn manifest(in_dir: &Path, params: &Parameters) -> BTreeMap<String, ReactionBuilder> {
    section("Manifest");

    let reaction_builders = map::<ReactionBuilder>(&in_dir.join("reactions"), &params.reactions);

    reaction_builders
}

fn building() {
    section("Building");
}
