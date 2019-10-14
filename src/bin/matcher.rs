//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::phy::{kin::Properties as KinProp, opt::Properties as OptProp, Material};

fn main() {
    println!("Hello world!");

    let _stratum_corneum = Material::new(
        Some(OptProp::new(1.5, 100.0 * 1e3, 0.630 * 1e3, 0.86)),
        Some(KinProp::new(1.0)),
    );
    let _living_epidermis =
        Material::new(Some(OptProp::new(1.34, 45.0 * 1e3, 0.872 * 1e3, 0.8)), None);
    let _papillary_dermis =
        Material::new(Some(OptProp::new(1.4, 30.0 * 1e3, 0.106 * 1e3, 0.8)), None);
    let _upper_blood_net_dermis =
        Material::new(Some(OptProp::new(1.39, 35.0 * 1e3, 0.142 * 1e3, 0.8)), None);
    let _reticular_dermis =
        Material::new(Some(OptProp::new(1.4, 25.0 * 1e3, 0.106 * 1e3, 0.8)), None);
    let _deep_blood_net_dermis =
        Material::new(Some(OptProp::new(1.38, 30.0 * 1e3, 0.164 * 1e3, 0.8)), None);
    let _subcutaneous_fat =
        Material::new(Some(OptProp::new(1.44, 5.0 * 1e3, 0.104 * 1e3, 0.8)), None);
}
