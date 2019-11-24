//! Region structure.

use crate::{
    geom::shape::{Mesh, ProtoMesh},
    json,
    sim::evolve::{ProtoState, State},
    world::MolMap,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Region structure implementation.
/// Used to initialise the initial state of cells.
#[derive(Debug)]
pub struct Region {
    /// Surface mesh.
    mesh: Mesh,
    /// Initial state.
    state: State,
}

impl Region {
    /// Construct a new instance.
    pub fn new(mesh: Mesh, state: State) -> Self {
        Self { mesh, state }
    }

    /// Build an instance from a proto-region.
    pub fn build(mesh_dir: &Path, mol_map: &MolMap, proto_region: &ProtoRegion) -> Self {
        Self::new(
            Mesh::build(mesh_dir, proto_region.mesh()),
            State::build(mol_map, proto_region.state()),
        )
    }

    /// Reference the surface mesh.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Reference the state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

/// Proto-Region structure implementation.
/// Stores information required to build a region.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoRegion {
    /// Proto-mesh.
    mesh: ProtoMesh,
    /// Proto-state.
    state: ProtoState,
}

impl ProtoRegion {
    /// Construct a new instance.
    pub fn new(mesh: ProtoMesh, state: ProtoState) -> Self {
        Self { mesh, state }
    }

    /// Reference the proto-mesh.
    pub fn mesh(&self) -> &ProtoMesh {
        &self.mesh
    }

    /// Reference the proto-state.
    pub fn state(&self) -> &ProtoState {
        &self.state
    }
}

json!(ProtoRegion);
