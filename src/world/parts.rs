//! Parts world sub-module.

pub mod interfaces;
pub mod interfaces_builder;
pub mod materials;
pub mod materials_builder;
pub mod meshes_builder;
pub mod named;
pub mod reactions;
pub mod reactions_builder;
pub mod species;
pub mod species_builder;

pub use self::interfaces::*;
pub use self::materials::*;
pub use self::named::*;
pub use self::reactions::*;
pub use self::species::*;

use crate::file::io::Load;
use contracts::pre;
use log::info;
use std::{collections::HashMap, path::Path};

#[pre(dir.is_dir())]
#[pre(!desc.is_empty())]
fn load_map<T: Load>(dir: &Path, names: &[String], desc: &str) -> HashMap<String, T> {
    let mut map = HashMap::with_capacity(names.len());

    for name in names {
        let path = dir.join(format!("{}.json", name));
        info!("Loading {}: {}", desc, name);
        map.insert(name.clone(), T::load(&path));
    }

    map
}
