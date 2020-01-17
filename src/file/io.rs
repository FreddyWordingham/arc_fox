//! Input/Output file sub-module.

pub mod load;
pub mod save;

pub use self::{load::*, save::*};

/// Type can be read and written between the binary and stored json.
#[macro_export]
macro_rules! rw_json {
    ($type:ident) => {
        impl crate::file::io::Save for $type {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                crate::file::io::as_json(self, path);
            }
        }

        impl crate::file::io::Load for $type {
            #[inline]
            #[must_use]
            fn load(path: &std::path::Path) -> Self {
                crate::file::io::from_json(path)
            }
        }
    };
}
