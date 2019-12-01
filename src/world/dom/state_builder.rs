//! Interface-Builder structure.

use crate::json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Interface-Builder structure implementation.
/// Used to build interfaces.
#[derive(Debug, Deserialize, Serialize)]
pub struct StateBuilder {
    /// Species concentration-map.
    pub concs: Option<HashMap<String, f64>>,
    /// Species source-map.
    pub sources: Option<HashMap<String, f64>>,
}

json!(StateBuilder);
