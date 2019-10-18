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
    /// List of all used materials.
    mat_list: Vec<String>,
    /// List of boundaries.
    bound_list: Vec<Boundary>,
    /// Domain grid.
    grid: Grid,
}

impl Matcher {
    /// Construct a new instance.
    #[pre(!mat_list.is_empty())]
    pub fn new(mat_list: Vec<String>, bound_list: Vec<Boundary>, grid: Grid) -> Self {
        let mut man = Self {
            mat_list,
            bound_list,
            grid,
        };
        man.cleanse();

        man
    }

    /// Matcher model example.
    pub fn example() -> Self {
        let mat_list = vec![
            "air".to_string(),
            "stratumcorneum".to_string(),
            "livingepidermis".to_string(),
            "papillarydermis".to_string(),
            "upperbloodnetdermis".to_string(),
            "reticulardermis".to_string(),
            "deepbloodnetdermis".to_string(),
            "subcutaneousfat".to_string(),
            "othertissues".to_string(),
        ];

        let bound_list = vec![
            Boundary::new(
                "air_stratumcorneum".to_string(),
                "air".to_string(),
                "stratumcorneum".to_string(),
                None,
            ),
            Boundary::new(
                "stratumcorneum_livingepidermis".to_string(),
                "stratumcorneum".to_string(),
                "livingepidermis".to_string(),
                None,
            ),
            Boundary::new(
                "livingepidermis_papillarydermis".to_string(),
                "livingepidermis".to_string(),
                "papillarydermis".to_string(),
                None,
            ),
            Boundary::new(
                "papillarydermis_upperbloodnetdermis".to_string(),
                "papillarydermis".to_string(),
                "upperbloodnetdermis".to_string(),
                None,
            ),
            Boundary::new(
                "upperbloodnetdermis_reticulardermis".to_string(),
                "upperbloodnetdermis".to_string(),
                "reticulardermis".to_string(),
                None,
            ),
            Boundary::new(
                "reticulardermis_deepbloodnetdermis".to_string(),
                "reticulardermis".to_string(),
                "deepbloodnetdermis".to_string(),
                None,
            ),
            Boundary::new(
                "deepbloodnetdermis_subcutaneousfat".to_string(),
                "deepbloodnetdermis".to_string(),
                "subcutaneousfat".to_string(),
                None,
            ),
            Boundary::new(
                "subcutaneousfat_othertissues".to_string(),
                "subcutaneousfat".to_string(),
                "othertissues".to_string(),
                None,
            ),
        ];

        let grid = Grid::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [16, 16, 16]);

        Self::new(mat_list, bound_list, grid)
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
