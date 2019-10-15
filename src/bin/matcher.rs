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
use std::{collections::HashMap, env::args, path::Path};

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
    // let man = Matcher::example();
    // man.save(Path::new("new.json"));
    let _mat_map = load_mat_map(man.mat_list());
}

/// Load the given list of materials to the hashmap.
fn load_mat_map(mat_list: &Vec<String>) -> HashMap<String, Material> {
    let mat_dir = materials();
    let mut mat_map = HashMap::with_capacity(mat_list.len());

    for name in mat_list {
        info!("Loading {} material...", name);
        let path = mat_dir.join(format!("{}.json", name));

        mat_map.insert(name.clone(), Material::load(&path));
    }

    mat_map
}
