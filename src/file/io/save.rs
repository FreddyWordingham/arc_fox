//! Savable trait.

use crate::util::list::dimension::Cartesian::{X, Y, Z};
use ndarray::{Array2, Array3};
use netcdf::{variable::Numeric, File};
use serde::Serialize;
use serde_json::to_string;
use std::{fmt::Debug, fs::write, path::Path};

/// Types implementing this trait can be saved to file.
pub trait Save: Debug {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the type in json format.
#[inline]
pub fn as_json<T: Serialize>(instance: &T, path: &Path) {
    write(
        path,
        to_string(instance).expect("Unable to serialise object."),
    )
    .expect("Unable to write json file.");
}

impl<T: Debug + Numeric> Save for Array2<T> {
    fn save(&self, path: &Path) {
        let mut file = File::create(path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        file.root_mut()
            .add_dimension(dim1_name, shape[X as usize])
            .expect("Unable to add X dimension.");
        let dim2_name = "y";
        file.root_mut()
            .add_dimension(dim2_name, shape[Y as usize])
            .expect("Unable to add Y dimension.");

        let var = file
            .root_mut()
            .add_variable::<T>("data", &[dim1_name, dim2_name])
            .expect("Unable to add dataslice entry.");
        var.put_values(
            self.as_slice().expect("Unable to write dataslice values."),
            None,
            None,
        )
        .expect("Unable to store dataslice values.");
    }
}

impl<T: Debug + Numeric> Save for Array3<T> {
    fn save(&self, path: &Path) {
        let mut file = File::create(path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        file.root_mut()
            .add_dimension(dim1_name, shape[X as usize])
            .expect("Unable to add X dimension.");
        let dim2_name = "y";
        file.root_mut()
            .add_dimension(dim2_name, shape[Y as usize])
            .expect("Unable to add Y dimension.");
        let dim3_name = "z";
        file.root_mut()
            .add_dimension(dim3_name, shape[Z as usize])
            .expect("Unable to add Z dimension.");

        let var = file
            .root_mut()
            .add_variable::<T>("data", &[dim1_name, dim2_name, dim3_name])
            .expect("Unable to add datacube entry.");
        var.put_values(
            self.as_slice().expect("Unable to write datacube values."),
            None,
            None,
        )
        .expect("Unable to store datacube values.");
    }
}
