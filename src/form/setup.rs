//! Setup form structure.

use crate::json;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Universe setup information.
#[derive(Serialize, Deserialize)]
pub struct Setup {
    /// Resolution of the grid.
    pub resolution: [usize; 3],
    /// Half widths of the universe.
    pub half_widths: [f64; 3],
}

impl Setup {
    /// Construct an example instance.
    pub fn example() -> Self {
        Self {
            resolution: [17, 17, 17],
            half_widths: [1.0, 1.0, 1.0],
        }
    }
}

json!(Setup);
