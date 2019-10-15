//! Matcher model input

use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Matcher manifest input form.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matcher {
    /// Current working directory.
    current_working_dir: PathBuf,
}

impl Matcher {
    pub fn new(current_working_dir: PathBuf) -> Self {
        Self {
            current_working_dir,
        }
    }
}

impl Saveable for Matcher {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Matcher {
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
