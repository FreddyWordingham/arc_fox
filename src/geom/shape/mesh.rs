//! Mesh structure.

use super::{
    super::{ProtoTransform, Transform},
    Aabb, Triangle,
};
use crate::{json, list::alphabet::Greek::Alpha};
use contracts::pre;
use nalgebra::Similarity3;
use serde::{Deserialize, Serialize};

/// Mesh structure implementation.
/// Forms the surface of the majority of complex components.
#[derive(Debug)]
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new instance.
    #[pre(!tris.is_empty())]
    pub fn new(tris: Vec<Triangle>) -> Self {
        Self {
            aabb: Self::init_aabb(&tris),
            tris,
        }
    }

    /// Initialise the axis-aligned bounding box for the given list of triangles.
    fn init_aabb(tris: &Vec<Triangle>) -> Aabb {
        let mut mins = tris[0].verts()[Alpha as usize];
        let mut maxs = mins;

        for t in tris.iter() {
            for v in t.verts().iter() {
                for i in 0..3 {
                    if mins[i] > v[i] {
                        mins[i] = v[i];
                    } else if maxs[i] < v[i] {
                        maxs[i] = v[i];
                    }
                }
            }
        }

        Aabb::new(mins, maxs)
    }
}

impl Transform for Mesh {
    fn transform(&mut self, trans: &Similarity3<f64>) {
        for tri in self.tris.iter_mut() {
            tri.transform(trans);
        }

        self.aabb = Self::init_aabb(&self.tris);
    }
}

/// Proto-Transform structure implementation.
/// Stores information required to build a mesh.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoMesh {
    /// Mesh name.
    name: String,
    /// Optional transform.
    trans: Option<ProtoTransform>,
}

impl ProtoMesh {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    pub fn new(name: String, trans: Option<ProtoTransform>) -> Self {
        Self { name, trans }
    }

    /// Build a mesh.
    pub fn build(&self, mesh_dir: &Path) -> Mesh {
        let tris = Vec::load(&mesh_dir.join(format!("{}.obj", self.name)));
        let mut mesh = Mesh::new(tris);

        if let Some(trans) = &self.trans {
            let trans = trans.build();
            mesh.transform(&trans);
        }

        mesh
    }
}

json!(ProtoMesh);
