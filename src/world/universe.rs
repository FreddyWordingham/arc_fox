//! Universe structure.

use crate::{base::Resolution, json, mat::ProtoInterface};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Universe structure implementation.
#[derive(Debug)]
pub struct Universe {
    // Fields.
}

impl Universe {
    /// Construct a new instance.
    #[pre(true)]
    pub fn new() -> Self {
        Self {}
    }
}

/// Proto-Universe structure implementation.
/// Used to build universes.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoUniverse {
    /// Grid resolution.
    res: Resolution,
    /// Interfaces.
    inters: Vec<ProtoInterface>,
}

impl ProtoUniverse {
    /// Construct a new instance.
    #[pre(!inters.is_empty())]
    pub fn new(res: Resolution, inters: Vec<ProtoInterface>) -> Self {
        Self { res, inters }
    }
}

json!(ProtoUniverse);
