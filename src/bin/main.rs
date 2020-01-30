//! Main function.

use arc::{
    args,
    file::{Load, Verse as FileVerse},
    report,
    util::{banner, exec, io_dirs},
};
use attr::form;
use colog;
use log::info;
use std::path::{Path, PathBuf};

#[form]
struct Parameters {
    num_threads: usize,
    verse: FileVerse,
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
    let verse = params.verse.form(&in_dir);

    banner::section("Pre-Build");
    info!("Universe contents:\n{}", verse);

    banner::section("Building");

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

// fn load(in_dir: &Path, params: &Parameters) {
//     let materials = filter_materials(&params.interfaces);
//     let materials = load_set::<Material>(&in_dir.join("materials"), &materials, "json");

//     let interfaces = build_interfaces(&in_dir.join("meshes"), &params.interfaces, &materials);

//     Verse::new(materials, interfaces);
// }
