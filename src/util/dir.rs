//! Directory path constants.

use contracts::pre;
use std::{
    env::set_current_dir,
    fmt::{Display, Formatter, Result},
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Directory information storage.
pub struct Dir {
    /// Current working directory.
    cwd: PathBuf,
    /// Output directory.
    out: PathBuf,
    /// Materials resources directory.
    mats: PathBuf,
    /// Mesh resources directory.
    meshes: PathBuf,
}

impl Dir {
    /// Construct a new instance.
    #[pre(cwd.is_dir())]
    #[pre(mats.is_dir())]
    #[pre(meshes.is_dir())]
    pub fn new(cwd: PathBuf, out: PathBuf, mats: PathBuf, meshes: PathBuf) -> Self {
        set_current_dir(&cwd).expect("Unable to set the current working directory!");
        create_dir_all(&out).unwrap();

        Self {
            cwd,
            out,
            mats,
            meshes,
        }
    }

    /// Reference the current working directory.
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    /// Reference the target output directory.
    pub fn out(&self) -> &Path {
        &self.out
    }

    /// Reference the material resources directory.
    pub fn mats(&self) -> &Path {
        &self.mats
    }

    /// Reference the meshes resources directory.
    pub fn meshes(&self) -> &Path {
        &self.meshes
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Cwd      : {}", self.cwd.display())?;
        writeln!(f, "Output   : {}", self.out.display())?;
        writeln!(f, "Materials: {}", self.mats.display())?;
        write!(f, "Meshes   : {}", self.meshes.display())
    }
}
