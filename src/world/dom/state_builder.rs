//! Interface-Builder structure.

use crate::json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Interface-Builder structure implementation.
/// Used to build interfaces.
#[derive(Debug, Deserialize, Serialize)]
pub struct StateBuilder {
    /// Species concentration-map.
    concs: HashMap<String, f64>,
    /// Species source-map.
    sources: HashMap<String, f64>,
}

json!(StateBuilder);
