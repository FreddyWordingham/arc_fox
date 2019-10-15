//! Matcher model input

use crate::file::{as_json, from_json, Loadable, Saveable};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Matcher manifest input form.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matcher {
    /// Number of threads.
    num_threads: usize,
}

impl Matcher {
    /// Construct a new instance.
    pub fn new(num_threads: usize) -> Self {
        Self { num_threads }
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
