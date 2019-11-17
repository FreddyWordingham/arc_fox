//! Univariate enumeration.

use crate::{
    json,
    world::{map::index_of_key, MolMap},
};
use contracts::pre;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Univariate enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug)]
pub enum Rate {
    /// Constant value. f(cs) = C
    ZerothOrder(f64),
    /// Proportional to one reactant concentration.
    FirstOrder(f64, usize),
    /// Proportional to two reactant concentrations.
    SecondOrder(f64, usize, usize),
}

impl Rate {
    /// Construct a new zeroth order instance.
    #[pre(k > 0.0)]
    pub fn new_zeroth_order(k: f64) -> Self {
        Rate::ZerothOrder(k)
    }

    /// Construct a new first order instance.
    #[pre(k > 0.0)]
    pub fn new_first_order(k: f64, mol_ind: usize) -> Self {
        Rate::FirstOrder(k, mol_ind)
    }

    /// Construct a new second order instance.
    #[pre(k > 0.0)]
    pub fn new_second_order(k: f64, mol_ind_0: usize, mol_ind_1: usize) -> Self {
        Rate::SecondOrder(k, mol_ind_0, mol_ind_1)
    }

    /// Build an instance from a proto-reaction.
    #[pre(!mol_map.is_empty())]
    pub fn build(mol_map: &MolMap, proto_rate: &ProtoRate) -> Self {
        match proto_rate {
            ProtoRate::ZerothOrder(k) => Self::new_zeroth_order(*k),
            ProtoRate::FirstOrder(k, c_id) => {
                Self::new_first_order(*k, index_of_key(mol_map, c_id))
            }
            ProtoRate::SecondOrder(k, c_id_0, c_id_1) => Self::new_second_order(
                *k,
                index_of_key(mol_map, c_id_0),
                index_of_key(mol_map, c_id_1),
            ),
        }
    }

    /// Calculate the result of the formula.
    pub fn res(&self, concs: &Array1<f64>) -> f64 {
        match self {
            Rate::ZerothOrder(k) => -*k,
            Rate::FirstOrder(k, mol_ind) => -*k * concs[*mol_ind],
            Rate::SecondOrder(k, mol_ind_0, mol_ind_1) => {
                -*k * concs[*mol_ind_0] * concs[*mol_ind_1]
            }
        }
    }
}

/// Proto-Rate structure implementation.
/// Stores information required to build a rate.
#[derive(Debug, Serialize, Deserialize)]
pub enum ProtoRate {
    /// Constant value. f(cs) = C
    ZerothOrder(f64),
    /// Proportional to one reactant concentration.
    FirstOrder(f64, String),
    /// Proportional to two reactant concentrations.
    SecondOrder(f64, String, String),
}

impl ProtoRate {
    /// Construct a new proto-zeroth order instance.
    #[pre(k > 0.0)]
    pub fn new_zeroth_order(k: f64) -> Self {
        ProtoRate::ZerothOrder(k)
    }

    /// Construct a new proto-first order instance.
    #[pre(k > 0.0)]
    #[pre(!mol_id.is_empty())]
    pub fn new_first_order(k: f64, mol_id: String) -> Self {
        ProtoRate::FirstOrder(k, mol_id)
    }

    /// Construct a new proto-second order instance.
    #[pre(k > 0.0)]
    #[pre(!mol_id_0.is_empty())]
    #[pre(!mol_id_1.is_empty())]
    pub fn new_second_order(k: f64, mol_id_0: String, mol_id_1: String) -> Self {
        ProtoRate::SecondOrder(k, mol_id_0, mol_id_1)
    }

    /// Retrieve a list of all rate dependencies.
    pub fn dependants(&self) -> Vec<&str> {
        match self {
            ProtoRate::ZerothOrder(_k) => vec![],
            ProtoRate::FirstOrder(_k, mol_id) => vec![mol_id],
            ProtoRate::SecondOrder(_k, mol_id_0, mol_id_1) => vec![mol_id_0, mol_id_1],
        }
    }
}

json!(ProtoRate);
