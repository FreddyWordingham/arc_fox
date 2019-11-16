//! File module.

pub mod load;
pub mod save;

pub use self::load::*;
pub use self::save::*;

/// Enable the structure to be serialised and de-serialised from a json file.
#[macro_export]
macro_rules! json {
    ($type:ident) => {
        use crate::file::{as_json, from_json, Load, Save};
        use std::path::Path;

        impl Save for $type {
            fn save(&self, path: &Path) {
                as_json(self, path);
            }
        }

        impl Load for $type {
            fn load(path: &Path) -> Self {
                from_json(path)
            }
        }
    };
}
