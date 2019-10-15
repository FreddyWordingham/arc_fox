//! Matcher model input

use super::super::Surface;
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

    /// Matcher model example.
    pub fn example() -> Self {
        let mat_list = vec![
            "air".to_string(),
            "stratumcorneum".to_string(),
            "livingepidermis".to_string(),
            "papillarydermis".to_string(),
            "upperbloodnetdermis".to_string(),
            "dermis".to_string(),
            "deepbloodnetdermis".to_string(),
            "subcutaneousfat".to_string(),
            "othertissues".to_string(),
        ];

        let surf_list = vec![
            Surface::new(
                "air_stratumcorneum".to_string(),
                "air".to_string(),
                "stratumcorneum".to_string(),
            ),
            Surface::new(
                "stratumcorneum_livingepidermis".to_string(),
                "stratumcorneum".to_string(),
                "livingepidermis".to_string(),
            ),
            Surface::new(
                "livingepidermis_papillarydermis".to_string(),
                "livingepidermis".to_string(),
                "papillarydermis".to_string(),
            ),
            Surface::new(
                "papillarydermis_upperbloodnetdermis".to_string(),
                "papillarydermis".to_string(),
                "upperbloodnetdermis".to_string(),
            ),
            Surface::new(
                "upperbloodnetdermis_dermis".to_string(),
                "upperbloodnetdermis".to_string(),
                "dermis".to_string(),
            ),
            Surface::new(
                "dermis_deepbloodnetdermis".to_string(),
                "dermis".to_string(),
                "deepbloodnetdermis".to_string(),
            ),
            Surface::new(
                "deepbloodnetdermis_subcutaneousfat".to_string(),
                "deepbloodnetdermis".to_string(),
                "subcutaneousfat".to_string(),
            ),
            Surface::new(
                "subcutaneousfat_othertissues".to_string(),
                "subcutaneousfat".to_string(),
                "othertissues".to_string(),
            ),
        ];

        Self::new(mat_list, surf_list)
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
