//! Matcher model input

use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use log::warn;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Surface setup information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Surface {
    /// Mesh surface.
    mesh: String,
    /// Inside material.
    inside: String,
    /// Outside material.
    outside: String,
}

impl Surface {
    /// Construct a new instance.
    pub fn new(mesh: String, inside: String, outside: String) -> Self {
        Self {
            mesh,
            inside,
            outside,
        }
    }
}

/// Matcher manifest input form.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matcher {
    /// List of all used materials.
    mat_list: Vec<String>,
    /// List of surfaces.
    surf_list: Vec<Surface>,
}

impl Matcher {
    /// Construct a new instance.
    #[pre(!mat_list.is_empty())]
    pub fn new(mat_list: Vec<String>, surf_list: Vec<Surface>) -> Self {
        let mut man = Self {
            mat_list,
            surf_list,
        };
        man.cleanse();

        man
    }

    /// Cleanse itself before use.
    fn cleanse(&mut self) {
        let pre_len = self.mat_list.len();

        self.mat_list.sort();
        self.mat_list.dedup();

        if self.mat_list.len() != pre_len {
            warn!(
                "{} materials were removed during manifest cleansing.",
                pre_len - self.mat_list.len()
            );
        }
    }

    /// Reference the material list.
    pub fn mat_list(&self) -> &Vec<String> {
        &self.mat_list
    }

    /// Reference the surfacce list.
    pub fn surf_list(&self) -> &Vec<Surface> {
        &self.surf_list
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
