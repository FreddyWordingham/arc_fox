//! Archive structure.

use super::Record;
use ndarray::Array3;

/// Archive datacube alias type.
pub type Archive = Array3<Record>;
