//! Directory proto-structure.

// use crate::file::{as_json, from_json, Loadable, Saveable};
use crate::{form::Manifestable, util::Dir};
use serde::{Deserialize, Serialize};

/// Proto-dir structure used to manifest dir structures.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoDir {
    /// Optional target current working directory.
    cwd: Option<String>,
}

impl Manifestable<Dir> for ProtoDir {}
