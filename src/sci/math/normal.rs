//! Normal trait.

use nalgebra::Vector3;
use std::fmt::Debug;

/// Types implementing this trait may be checked for normality.
pub trait Normal: Debug {
    /// Determine if the value is normalised.
    fn is_normal(&self) -> bool;
}

impl Normal for f64 {
    fn is_normal(&self) -> bool {
        0.0 <= *self && *self <= 1.0
    }
}

impl Normal for Vector3<f64> {
    fn is_normal(&self) -> bool {
        (self.magnitude_squared() - 1.0).abs() < 1.0e-6
    }
}
