//! Saveable to file trait.

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
    fn save(&self, path: &Path) {
        let mut file = File::create(&path).expect("Unable to create netcdf file!");

        let shape = self.shape();

        let dim1_name = "x";
        let dim2_name = "y";
        let dim3_name = "z";
        file.root_mut().add_dimension(dim1_name, shape[0]).unwrap();
        file.root_mut().add_dimension(dim2_name, shape[1]).unwrap();
        file.root_mut().add_dimension(dim3_name, shape[2]).unwrap();

        let var_name = "varstuff";
        let var = &mut file
            .root_mut()
            .add_variable::<T>(var_name, &[dim1_name, dim2_name, dim3_name])
            .unwrap();
        var.put_values(self.as_slice().unwrap(), None, None)
            .unwrap();
    }
}