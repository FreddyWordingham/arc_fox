//! Saveable to file trait.

use crate::dim::Cartesian::{X, Y, Z};
use contracts::pre;
use ndarray::Array3;
use netcdf::{variable::Numeric, File};
use serde::Serialize;
use serde_json::to_string;
use std::{fs::write, path::Path};

/// Types implementing this trait can be saved to a file on disk.
pub trait Saveable {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the object in json format if it implements the Serialize trait.
pub fn as_json<T: Serialize>(obj: &T, path: &Path) {
    write(path, to_string(obj).expect("Unable to serialise object!"))
        .expect("Unable to write json file!")
}

impl<T: Numeric> Saveable for Array3<T> {
    #[pre(self.shape().iter().all(|x| *x > 0))]
    fn save(&self, path: &Path) {
        let mut file = File::create(&path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        let dim2_name = "y";
        let dim3_name = "z";
        file.root_mut()
            .add_dimension(dim1_name, shape[X as usize])
            .expect("Unable to add dimension.");
        file.root_mut()
            .add_dimension(dim2_name, shape[Y as usize])
            .expect("Unable to add dimension.");
        file.root_mut()
            .add_dimension(dim3_name, shape[Z as usize])
            .expect("Unable to add dimension.");

        let var = &mut file
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
