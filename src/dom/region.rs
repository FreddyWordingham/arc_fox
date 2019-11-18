//! Region structure.

use crate::{
    geom::shape::{Mesh, ProtoMesh},
    json,
    world::{map::index_of_key, MolMap},
};
use contracts::pre;
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Region structure implementation.
/// Used to initialise the initial state of cells.
#[derive(Debug)]
pub struct Region {
    /// Surface mesh.
    mesh: Mesh,
    /// Initial concentrations.
    concs: Array1<f64>,
    /// Source terms.
    sources: Array1<f64>,
}

impl Region {
    /// Construct a new instance.
    #[pre(sources.iter().all(|x| *x >= 0.0))]
    #[pre(concs.len() == sources.len())]
    pub fn new(mesh: Mesh, concs: Array1<f64>, sources: Array1<f64>) -> Self {
        Self {
            mesh,
            concs,
            sources,
        }
    }

    /// Build an instance from a proto-region.
    pub fn build(mesh_dir: &Path, mol_map: &MolMap, proto_region: &ProtoRegion) -> Self {
        let mut concs = Array1::zeros(mol_map.len());
        let mut sources = Array1::zeros(mol_map.len());
        for (id, (init_conc, source)) in proto_region.init_conc_sources().iter() {
            let index = index_of_key(mol_map, id);
            concs[index] = *init_conc;
            sources[index] = *source;
        }

        Self::new(Mesh::build(mesh_dir, proto_region.mesh()), concs, sources)
    }

    /// Reference the surface mesh.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Reference the initial concentrations.
    pub fn concs(&self) -> &Array1<f64> {
        &self.concs
    }

    /// Reference the source terms.
    pub fn sources(&self) -> &Array1<f64> {
        &self.sources
    }
}

/// Proto-Region structure implementation.
/// Stores information required to build a region.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtoRegion {
    /// Proto-mesh.
    mesh: ProtoMesh,
    /// Map of initial concentrations and sources.
    init_conc_sources: HashMap<String, (f64, f64)>,
}

impl ProtoRegion {
    /// Construct a new instance.
    pub fn new(mesh: ProtoMesh, init_conc_sources: HashMap<String, (f64, f64)>) -> Self {
        Self {
            mesh,
            init_conc_sources,
        }
    }

    /// Reference the proto-mesh.
    pub fn mesh(&self) -> &ProtoMesh {
        &self.mesh
    }

    /// Reference the map of initial concentrations and sources.
    pub fn init_conc_sources(&self) -> &HashMap<String, (f64, f64)> {
        &self.init_conc_sources
    }
}

json!(ProtoRegion);
