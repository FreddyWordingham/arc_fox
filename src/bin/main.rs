//! Main function.

use arc::{
    args,
    dom::{filter_materials, load_set},
    file::{Interface, Load},
    report,
    sim::Material,
    util::{banner, exec, io_dirs},
};
use attr::form;
use colog;
use log::info;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[form]
struct Parameters {
    num_threads: usize,
    interfaces: BTreeMap<String, Interface>,
}

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    banner::section("Loading");
    load(&in_dir, &params);

    banner::section("Building");

    banner::section("Pre-Flight");

    banner::section("Output");
    info!("Saving maps...");
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn prelude(params_path: &Path) -> Parameters {
    Parameters::load(&params_path)
}

fn load(in_dir: &Path, params: &Parameters) {
    let materials = filter_materials(&params.interfaces);
    let materials = load_set::<Material>(&in_dir.join("materials"), &materials, "json");
}
