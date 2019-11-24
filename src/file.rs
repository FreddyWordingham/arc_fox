//! File module.

pub mod load;
pub mod save;

pub use self::load::*;
pub use self::save::*;

/// Enable the structure to be serialised and de-serialised from a json file.
#[macro_export]
macro_rules! json {
    ($type:ident) => {
        impl crate::file::Save for $type {
            fn save(&self, path: &std::path::Path) {
                crate::file::as_json(self, path);
            }
        }

        impl crate::file::Load for $type {
            fn load(path: &std::path::Path) -> Self {
                crate::file::from_json(path)
            }
        }
    };
}
