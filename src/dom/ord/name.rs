//! Name alias.

use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Construct a new name type.
macro_rules! name {
    ($name:ident) => {
        #[json]
        #[derive(Clone, PartialOrd, Eq, PartialEq, Ord)]
        pub struct $name(String);

        impl Display for $name {
            #[inline]
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                write!(fmt, "{}", self.0)
            }
        }
    };
}

name!(Name);
