//! Input/Output file sub-module.

pub mod load;
pub mod save;

pub use self::load::*;
pub use self::save::*;

/// Enable a given type to be serialised and de-serialised from a json file.
#[macro_export]
macro_rules! json {
    ($type:ident) => {
        impl crate::file::io::Save for $type {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                crate::file::io::as_json(self, path);
            }
        }

        impl crate::file::io::Load for $type {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                crate::file::io::from_json(path)
            }
        }
    };
}
