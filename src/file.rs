//! File input and output traits and functions.

pub mod loadable;
pub mod saveable;

pub use self::loadable::*;
pub use self::saveable::*;

/// Enable the structure to be serialised and de-serialised from a json file.
#[macro_export]
macro_rules! json {
    ($type:ident) => {
        use crate::file::{as_json, from_json, Loadable, Saveable};

        impl Saveable for $type {
            fn save(&self, path: &Path) {
                as_json(self, path);
            }
        }

        impl Loadable for $type {
            fn load(path: &Path) -> Self {
                from_json(path)
            }
        }
    };
}
