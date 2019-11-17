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
    pub fn new_first_order(k: f64, ci: usize) -> Self {
        Rate::FirstOrder(k, ci)
    }

    /// Construct a new second order instance.
    #[pre(k > 0.0)]
    pub fn new_second_order(k: f64, ci_0: usize, ci_1: usize) -> Self {
        Rate::SecondOrder(k, ci_0, ci_1)
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
            Rate::FirstOrder(k, ci) => -*k * concs[*ci],
            Rate::SecondOrder(k, ci_0, ci_1) => -*k * concs[*ci_0] * concs[*ci_1],
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

json!(ProtoRate);
