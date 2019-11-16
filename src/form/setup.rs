//! Setup structure.

use crate::{base::Resolution, json};
use serde::{Deserialize, Serialize};

/// Setup structure implementation.
/// Load-time setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    /// Resolution of the grid.
    res: Resolution,
}

impl Setup {
    /// Construct a new instance.
    pub fn example() -> Self {
        Self {
            res: Resolution::new(9, 9, 9),
        }
    }
}

json!(Setup);
