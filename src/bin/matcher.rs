//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::{dir::materials, file::Loadable, phy::Material};
use std::{collections::HashMap, path::Path};

fn main() {
    println!("Hello world!");

    let mat_dir = materials();

    let _materials = load_materials(
        &mat_dir,
        vec![
            "stratum_corneum",
            "living_epidermis",
            "papillary_dermis",
            "upper_blood_net_dermis",
            "reticular_dermis",
            "deep_blood_net_dermis",
            "subcutaneous_fat",
        ],
    );
}

fn load_materials(mat_dir: &Path, mat_list: Vec<&str>) -> HashMap<String, Material> {
    println!("Loading materials from: {}", mat_dir.display());

    let mut materials = HashMap::new();
    for name in mat_list {
        let path = mat_dir.join(format!("{}.json", name));
        println!("Loading {}", path.display());

        materials.insert(name.to_string(), Material::load(&path));
    }
    println!("Materials loaded!");

    materials
}
