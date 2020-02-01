//! Main function.

use arc::{
    args,
    file::{Grid as FileGrid, Load, Save, Verse as FileVerse},
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
    grid: FileGrid,
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

    banner::section("Building");
    let grid = params.grid.form(&verse);

    banner::section("Overview");
    info!("Universe contents:\n{}", verse);

    banner::section("Analysis");
    info!("Generating material maps...");
    let mat_maps = grid.mat_set(verse.mats());
    let total_cells = grid.cells().len();
    for (name, map) in mat_maps.map() {
        println!(
            "{:<32}\t{:<10}\t{}%",
            format!("{}:", name),
            map.sum(),
            map.sum() / total_cells as f64 * 100.0
        );
    }

    banner::section("Output");
    info!("Saving maps...");
    for (name, map) in mat_maps.map() {
        map.save(&out_dir.join(format!("{}_map.nc", name)));
    }

    banner::section("Finished");
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
