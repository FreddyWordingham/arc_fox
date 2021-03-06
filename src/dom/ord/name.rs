//! Name alias.

use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Construct a new name type.
macro_rules! name {
    ($name:ident) => {
        #[json]
        #[derive(Clone, PartialOrd, Eq, PartialEq, Ord)]
        pub struct $name(String);

        impl $name {
            #[inline]
            #[must_use]
            pub fn new(string: &str) -> Self {
                Self {
                    0: string.to_string(),
                }
            }

            #[inline]
            #[must_use]
            pub fn str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                write!(fmt, "{}", self.0)
            }
        }
    };
}

name!(Name);
