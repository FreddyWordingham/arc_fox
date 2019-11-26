//! Material structure.

use crate::{
    json,
    math::Formula,
    opt::Environment,
    sim::evolve::{ProtoState, State},
    util::Range,
    world::MolMap,
};
use contracts::pre;
use serde::{Deserialize, Serialize};

/// Material structure implementation.
/// Stores the local optical, diffusive and kinematic information.
#[derive(Debug)]
pub struct Material {
    /// Range of valid wavelengths.
    range: Range,
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient. [m^-1]
    scat_coeff: Formula,
    /// Absorption coefficient. [m^-1]
    abs_coeff: Formula,
    /// Shift coefficient. [m^-1]
    shift_coeff: Formula,
    /// Asymmetry parameter.
    asym: Formula,
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Initial state.
    state: State,
}

impl Material {
    /// Construct a new instance.
    #[pre(range.min() > 0.0)]
    #[pre(visc.is_none() || visc.unwrap() > 0.0)]
    pub fn new(
        range: Range,
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
        visc: Option<f64>,
        state: State,
    ) -> Self {
        Self {
            range,
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
            visc,
            state,
        }
    }

    /// Build an instance from a proto-material.
    pub fn build(proto_mat: &ProtoMaterial, mol_map: &MolMap) -> Self {
        let state = if let Some(proto_state) = &proto_mat.state {
            State::build(&mol_map, &proto_state)
        } else {
            State::new_empty(mol_map.len())
        };

        Self::new(
            proto_mat.range.clone(),
            proto_mat.ref_index.clone(),
            proto_mat.scat_coeff.clone(),
            proto_mat.abs_coeff.clone(),
            proto_mat.shift_coeff.clone(),
            proto_mat.asym.clone(),
            proto_mat.visc,
            state,
        )
    }

    /// Get the optical environment for a given wavelength.
    #[pre(self.range.contains(w))]
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.res(w),
            self.scat_coeff.res(w),
            self.abs_coeff.res(w),
            self.shift_coeff.res(w),
            self.asym.res(w),
        )
    }

    /// Reference the initial state.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Get the optional viscosity. [kg m s^-1]
    pub fn visc(&self) -> Option<f64> {
        self.visc
    }
}

/// Proto-Material structure implementation.
/// Stores information required to build a material.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoMaterial {
    /// Range of valid wavelengths.
    range: Range,
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient. [m^-1]
    scat_coeff: Formula,
    /// Absorption coefficient. [m^-1]
    abs_coeff: Formula,
    /// Shift coefficient. [m^-1]
    shift_coeff: Formula,
    /// Asymmetry parameter.
    asym: Formula,
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Initial state.
    state: Option<ProtoState>,
}

impl ProtoMaterial {
    /// Construct a new instance.
    #[pre(range.min() > 0.0)]
    #[pre(visc.is_none() || visc.unwrap() > 0.0)]
    pub fn new(
        range: Range,
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
        visc: Option<f64>,
        state: Option<ProtoState>,
    ) -> Self {
        Self {
            range,
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
            visc,
            state,
        }
    }
}

json!(ProtoMaterial);
