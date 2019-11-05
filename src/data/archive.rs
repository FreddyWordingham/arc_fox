//! Archive structure.

use super::Record;
use ndarray::Array3;

/// Record archive alias type.
pub type Archive = Array3<Record>;
