//! Resources directory paths.

use super::arc;
use contracts::post;
use std::path::{Path, PathBuf};

/// Get the root resources directory path.
#[post(ret.is_dir())]
pub fn root() -> PathBuf {
    Path::new(&arc().join("res")).to_path_buf()
}

macro_rules! get_subdir_path {
    ($name:ident) => {
        #[post(ret.is_dir())]
        pub fn $name() -> PathBuf {
            root().join(stringify!($name))
        }
    };
}

get_subdir_path!(materials);
get_subdir_path!(meshes);
get_subdir_path!(reactions);
get_subdir_path!(species);
