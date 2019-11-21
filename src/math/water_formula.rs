//! Formula enumeration.

use crate::{json, util::Range};
use serde::{Deserialize, Serialize};

/// Formula enumeration implementation.
/// Formulae that accept a single scalar value, and return a single scalar value.
#[derive(Debug, Serialize, Deserialize)]

pub enum Formula {
    /// Constant value. f(x) = C
    Water_abs(f64),
}
impl Formula{
    //Construct water absorption formula.
    pub fn new_water_abs(w: f64) -> Self{
        Formula::Water_abs(w)
    }

    //Calculate the result of the formula.
    pub fn out(&self, _x: f64) -> f64{
        match self{
            Formula::Water_abs(w) => if w == 830e-9{
                2.0
            }
            else {
                10.0
            }
        }
    }
}


json!(Formula);
