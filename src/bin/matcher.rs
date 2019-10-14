//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::phy::{kin::Properties as KinProp, opt::Properties as OptProp, Material};
use std::collections::HashMap;

fn main() {
    println!("Hello world!");

    let mut _materials = load_materials(vec![
        "stratum_corneum",
        "living_epidermis",
        "papillary_dermis",
        "upper_blood_net_dermis",
        "reticular_dermis",
        "deep_blood_net_dermis",
        "subcutaneous_fat",
    ]);
}

fn load_materials(_mat_list: Vec<&str>) -> HashMap<String, Material> {
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

    materials
}
