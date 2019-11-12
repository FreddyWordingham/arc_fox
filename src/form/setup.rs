//! Setup form structure.

use crate::json;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Universe setup information.
#[derive(Serialize, Deserialize)]
pub struct Setup {
    /// Resolution of the grid.
    res: [usize; 3],
}

json!(Setup);
