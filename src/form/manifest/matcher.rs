//! Matcher model input

use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Matcher manifest input form.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matcher {
    /// List of all used materials.
    mat_list: Vec<String>,
}

impl Matcher {
    /// Construct a new instance.
    #[pre(!mat_list.is_empty())]
    pub fn new(mat_list: Vec<String>) -> Self {
        let mut man = Self { mat_list };
        man.cleanse();

        man
    }

    /// Cleanse itself before use.
    fn cleanse(&mut self) {
        self.mat_list.sort();
        self.mat_list.dedup();
    }

    /// Reference the material list.
    pub fn mat_list(&self) -> &Vec<String> {
        &self.mat_list
    }
}

impl Saveable for Matcher {
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl Loadable for Matcher {
    fn load(path: &Path) -> Self {
        let mut man: Self = from_json(path);
        man.cleanse();

        man
    }
}
