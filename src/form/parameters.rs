//! Parameters structure.

use crate::{chem::ProtoReaction, dom::ProtoRegion, file::Load, json, mat::ProtoInterface};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Parameters structure implementation.
/// Load-time world building information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    /// Number threads to use.
    num_threads: usize,
    /// List of interfaces.
    interfaces: Vec<String>,
    /// Optional list of reactions to simulate.
    reactions: Option<Vec<String>>,
    /// Optional list of regions to initialise state.
    regions: Option<Vec<String>>,
}

impl Parameters {
    /// Construct a new instance.
    #[pre(num_threads > 0)]
    pub fn new(
        num_threads: usize,
        interfaces: Vec<&str>,
        reactions: Option<Vec<&str>>,
        regions: Option<Vec<&str>>,
    ) -> Self {
        let str_to_string = |list: Option<Vec<&str>>| -> Option<Vec<String>> {
            if let Some(rs) = list {
                return Some(rs.iter().map(|s| s.to_string()).collect());
            }

            None
        };

        let reactions = str_to_string(reactions);
        let regions = str_to_string(regions);

        Self {
            num_threads,
            interfaces: interfaces.iter().map(|s| s.to_string()).collect(),
            reactions,
            regions,
        }
    }

    /// Create the proto-interface-map.
    pub fn interfaces(&self, in_dir: &Path) -> HashMap<String, ProtoInterface> {
        Self::load_list::<ProtoInterface>(in_dir, &self.interfaces)
    }

    /// Create the proto-reaction-map.
    pub fn reactions(&self, in_dir: &Path) -> HashMap<String, ProtoReaction> {
        Self::load_list::<ProtoReaction>(in_dir, &self.reactions.as_ref().unwrap_or(&vec![]))
    }

    /// Create the proto-region-map.
    pub fn regions(&self, in_dir: &Path) -> HashMap<String, ProtoRegion> {
        Self::load_list::<ProtoRegion>(in_dir, &self.regions.as_ref().unwrap_or(&vec![]))
    }

    /// Load a list of a given type using json files.
    fn load_list<T: Load>(in_dir: &Path, list: &Vec<String>) -> HashMap<String, T> {
        let mut map = HashMap::with_capacity(list.len());

        for name in list.iter() {
            map.insert(
                name.clone(),
                T::load(&in_dir.join(format!("{}.json", name))),
            );
        }

        map
    }
}

json!(Parameters);
