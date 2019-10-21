//! Matcher model input

use super::super::{Boundary, Grid};
use crate::file::{as_json, from_json, Loadable, Saveable};
use contracts::pre;
use log::warn;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Matcher manifest input form.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matcher {
    // /// List of boundaries.
    // bound_list: Vec<Boundary>,
    /// Domain grid.
    grid: Grid,
}

impl Matcher {
    /// Construct a new instance.
    #[pre(!mat_list.is_empty())]
    pub fn new(grid: Grid) -> Self {
        let mut man = Self {
            mat_list,
            bound_list,
            grid,
        };
        man.cleanse();

        man
    }

    /// Reference the material list.
    pub fn mat_list(&self) -> &Vec<String> {
        &self.mat_list
    }

    /// Reference the boundary list.
    pub fn bound_list(&self) -> &Vec<Boundary> {
        &self.bound_list
    }

    /// Reference the grid information.
    pub fn grid(&self) -> &Grid {
        &self.grid
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
