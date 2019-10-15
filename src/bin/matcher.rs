//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::{
    file::Loadable,
    form::{load, manifest::Matcher},
    phy::Material,
    util::start_up,
};
use log::error;
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
    let _man = load::<Matcher>(input_file_path);

    // let mesh_dir = meshes();
    // let mat_dir = materials();

    // // Materials.
    // let _materials = load_materials(
    //     &mesh_dir,
    //     &mat_dir,
    //     vec![
    //         "stratum_corneum",
    //         "living_epidermis",
    //         "papillary_dermis",
    //         "upper_blood_net_dermis",
    //         "reticular_dermis",
    //         "deep_blood_net_dermis",
    //         "subcutaneous_fat",
    //     ],
    // );
}

/// Load the given list of materials.
fn load_materials(
    mesh_dir: &Path,
    mat_dir: &Path,
    mat_list: Vec<&str>,
) -> HashMap<String, Material> {
    println!("Loading materials from: {}", mat_dir.display());

    let mut materials = HashMap::new();
    for name in mat_list {
        let mesh_path = mesh_dir.join(format!("{}.obj", name));
        println!("Loading {}", mesh_path.display());

        let path = mat_dir.join(format!("{}.json", name));
        println!("Loading {}", path.display());

        materials.insert(name.to_string(), Material::load(&path));
    }
    println!("Materials loaded!");

    materials
}
