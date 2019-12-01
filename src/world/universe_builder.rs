//! Universe-Builder structure.

use crate::{
    sci::{
        chem::{ReactionBuilder, SpeciesBuilder},
        math::shape::{Aabb, Mesh},
    },
    world::{
        mat::{InterfaceBuilder, MaterialBuilder},
        parts::{
            interfaces_builder, materials_builder, meshes_builder, reactions_builder,
            species_builder,
        },
    },
};
use contracts::pre;
use std::{collections::HashMap, path::Path};

/// Universe-Builder structure implementation.
/// Used to build universes.
#[derive(Debug)]
pub struct UniverseBuilder {
    /// Domain boundary.
    pub dom: Aabb,
    /// Grid resolution.
    pub res: [usize; 3],
    /// Reaction-builder map.
    pub reactions: HashMap<String, ReactionBuilder>,
    /// Interface-builder map.
    pub interfaces: HashMap<String, InterfaceBuilder>,
    /// Mesh map.
    pub meshes: HashMap<String, Mesh>,
    /// Material-builder map.
    pub materials: HashMap<String, MaterialBuilder>,
    /// Species-builder map.
    pub species: HashMap<String, SpeciesBuilder>,
}

impl UniverseBuilder {
    /// Construct a new instance.
    #[pre(res.iter().all(|x| *x > 0))]
    #[pre(dir.is_dir())]
    #[pre(!interfaces.is_empty())]
    pub fn new(
        dom: Aabb,
        res: [usize; 3],
        dir: &Path,
        reactions: &[String],
        interfaces: &[String],
    ) -> Self {
        let reactions = reactions_builder::load(&dir.join("reactions"), reactions);
        let interfaces = interfaces_builder::load(&dir.join("interfaces"), interfaces);
        let meshes = meshes_builder::load(&dir.join("meshes"), &interfaces);
        let materials = materials_builder::load(&dir.join("materials"), &interfaces);
        let species = species_builder::load(&dir.join("species"), &reactions, &materials);

        Self {
            dom,
            res,
            reactions,
            interfaces,
            meshes,
            materials,
            species,
        }
    }
}
