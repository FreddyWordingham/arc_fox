//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::{
    dir::resources,
    file::Saveable,
    phy::{kin::Properties as KinProp, opt::Properties as OptProp, Material},
};
use std::{collections::HashMap, path::Path};

fn main() {
    println!("Hello world!");

    let res_dir = resources();

    let materials = load_materials(
        &res_dir.join("mats/"),
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

    for (name, mat) in materials {
        println!("Saving material: {}", name);
        mat.save(&res_dir.join("mats/").join(format!("{}.json", name)));
    }
}

fn load_materials(mat_dir: &Path, _mat_list: Vec<&str>) -> HashMap<String, Material> {
    println!("Loading materials from: {}", mat_dir.display());

    let mut materials = HashMap::new();

    materials.insert(
        "stratum_corneum".to_string(),
        Material::new(
            Some(OptProp::new(1.5, 100.0 * 1e3, 0.630 * 1e3, 0.86)),
            Some(KinProp::new(1.0)),
        ),
    );
    materials.insert(
        "living_epidermis".to_string(),
        Material::new(Some(OptProp::new(1.34, 45.0 * 1e3, 0.872 * 1e3, 0.8)), None),
    );
    materials.insert(
        "papillary_dermis".to_string(),
        Material::new(Some(OptProp::new(1.4, 30.0 * 1e3, 0.106 * 1e3, 0.8)), None),
    );
    materials.insert(
        "upper_blood_net_dermis".to_string(),
        Material::new(Some(OptProp::new(1.39, 35.0 * 1e3, 0.142 * 1e3, 0.8)), None),
    );
    materials.insert(
        "reticular_dermis".to_string(),
        Material::new(Some(OptProp::new(1.4, 25.0 * 1e3, 0.106 * 1e3, 0.8)), None),
    );
    materials.insert(
        "deep_blood_net_dermis".to_string(),
        Material::new(Some(OptProp::new(1.38, 30.0 * 1e3, 0.164 * 1e3, 0.8)), None),
    );
    materials.insert(
        "subcutaneous_fat".to_string(),
        Material::new(Some(OptProp::new(1.44, 5.0 * 1e3, 0.104 * 1e3, 0.8)), None),
    );

    println!("Materials loaded!");

    materials
}
