//! Parameters structure.

use crate::{
    chem::ProtoReaction,
    dom::ProtoGrid,
    file::Load,
    json,
    mat::ProtoInterface,
    world::{ProtoUniverse, Universe},
};
use contracts::pre;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Parameters structure implementation.
/// Load-time world building information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    /// Number threads to use.
    num_threads: usize,
    /// Grid.
    grid: ProtoGrid,
    /// List of interfaces.
    interfaces: Vec<String>,
    /// Optional list of reactions to simulate.
    reactions: Option<Vec<String>>,
}

impl Parameters {
    /// Construct a new instance.
    #[pre(num_threads > 0)]
    pub fn new(
        num_threads: usize,
        grid: ProtoGrid,
        interfaces: Vec<&str>,
        reactions: Option<Vec<&str>>,
    ) -> Self {
        let str_to_string = |list: Option<Vec<&str>>| -> Option<Vec<String>> {
            if let Some(rs) = list {
                return Some(rs.iter().map(|s| s.to_string()).collect());
            }

            None
        };

        let reactions = str_to_string(reactions);

        Self {
            num_threads,
            grid,
            interfaces: interfaces.iter().map(|s| s.to_string()).collect(),
            reactions,
        }
    }

    /// Get the number of threads.
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }

    /// Reference the proto-grid.
    pub fn grid(&self) -> &ProtoGrid {
        &self.grid
    }

    /// Create the proto-interface-map.
    #[pre(dir.is_dir())]
    fn interfaces(&self, dir: &Path) -> HashMap<String, ProtoInterface> {
        Self::load_list::<ProtoInterface>(&dir.join("interfaces"), &self.interfaces)
    }

    /// Create the proto-reaction-map.
    #[pre(dir.is_dir())]
    fn reactions(&self, dir: &Path) -> HashMap<String, ProtoReaction> {
        Self::load_list::<ProtoReaction>(
            &dir.join("reactions"),
            &self.reactions.as_ref().unwrap_or(&vec![]),
        )
    }

    /// Load a list of a given type using json files.
    fn load_list<T: Load>(dir: &Path, list: &Vec<String>) -> HashMap<String, T> {
        let mut map = HashMap::with_capacity(list.len());

        for name in list.iter() {
            map.insert(name.clone(), T::load(&dir.join(format!("{}.json", name))));
        }

        map
    }

    /// Manifest the input parameters into a universe.
    pub fn manifest(&self, dir: &Path) -> Universe {
        Universe::build(
            dir,
            &ProtoUniverse::new(self.grid.clone(), self.reactions(dir), self.interfaces(dir)),
            self.num_threads,
        )
    }
}

json!(Parameters);
