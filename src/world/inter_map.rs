//! Interface-map alias.

use crate::mat::Interface;
use std::collections::HashMap;

/// Interface-map alias.
pub type InterMap<'a> = HashMap<String, Interface<'a>>;
