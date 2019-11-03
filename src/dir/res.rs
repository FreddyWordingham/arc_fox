//! Resource path functions.

use super::res;
use contracts::post;
use std::path::PathBuf;

/// Get the resources directory path.
#[post(ret.is_dir())]
pub fn mats() -> PathBuf {
    res().join("mats")
}
