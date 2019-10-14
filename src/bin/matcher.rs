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

    let _stratum_corneum = Material::new(Some(OptProp::new(1.0, 0.1)), Some(KinProp::new(1.0)));
}
