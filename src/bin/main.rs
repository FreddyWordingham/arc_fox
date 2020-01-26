//! Cartographer testing binary.

use arc::{
    args,
    file::io::{Load, Save},
    report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use attr_mac::form;
use colog;
use log::info;
use std::path::{Path, PathBuf};

#[form]
struct Parameters {
    num_threads: usize,
    verse: arc::file::form::Verse,
    grid: arc::file::form::Grid,
}

pub fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    section("Loading");
    let verse = params.verse.form(&in_dir);

    section("Building");
    let grid = params.grid.form(&verse);

    let mut maps = Vec::with_capacity(verse.materials().map().len());
    for mat in verse.materials().map().keys() {
        maps.push((mat, grid.gen_mat_map(mat)));
    }

    section("Pre-Flight");
    report!(verse);

    section("Output");
    info!("Saving maps...");
    for (name, map) in maps {
        map.save(&out_dir.join(format!("{}.nc", name)));
        println!("{} -> {}", name, map.sum());
        // println!("{} -> {:?}", name, map);
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
    Parameters::load(&params_path)
}
