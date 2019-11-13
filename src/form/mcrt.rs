//! Setup form structure.

use crate::json;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monte-Carlo Radiative Transfer input form parameters.
#[derive(Serialize, Deserialize)]
pub struct Mcrt {}

impl Mcrt {
    /// Construct an example instance.
    pub fn example() -> Self {
        Self {}
    }
}

json!(Mcrt);
