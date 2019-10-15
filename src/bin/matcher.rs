//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::{
    dir::materials,
    file::Loadable,
    form::{load, manifest::Matcher},
    phy::Material,
    util::start_up,
};
use log::{error, info};
use std::{env::args, path::Path};

fn main() {
    // Start up.
    let (_cwd, _out_dir) = start_up(&Path::new("cwd"), &Path::new("out"));

    // Command line arguments.
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        error!("Required call:\n{} <path/to/manifest.json>", &args[0]);
        return;
    }
    let input_file_path = Path::new(&args[1]);

    // Manifest file.
    let man = load::<Matcher>(input_file_path);
    let _mats = load_mats(man.mat_list());
}

/// Load the given list of materials.
fn load_mats(mat_list: &Vec<String>) {
    let mat_dir = materials();

    for name in mat_list {
        info!("Loading {} material...", name);
        let path = mat_dir.join(format!("{}.json", name));
        let _mat = Material::load(&path);
    }
}
