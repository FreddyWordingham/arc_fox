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

impl<T: Serialize> Saveable for T {
    fn save(&self, path: &Path) {
        write(path, to_string(self).expect("Unable to serialise object!"))
            .expect("Unable to write json file!")
    }
}

/// Save a three-dimensional array as a hdf5 datacube.
pub fn save_as_netcdf<T: Numeric>(data: Vec<(&'static str, &Array3<T>)>, path: &Path) {
    let mut file = File::create(&path).expect("Unable to create netcdf file!");

    let shape = data[0].1.shape();

    let dim1_name = "x";
    let dim2_name = "y";
    let dim3_name = "z";
    file.root_mut().add_dimension(dim1_name, shape[0]).unwrap();
    file.root_mut().add_dimension(dim2_name, shape[1]).unwrap();
    file.root_mut().add_dimension(dim3_name, shape[2]).unwrap();

    for (name, d) in data {
        if d.shape() != shape {
            panic!("Shapes within the same datacube must match.");
        }

        let var = &mut file
            .root_mut()
            .add_variable::<T>(name, &[dim1_name, dim2_name, dim3_name])
            .unwrap();
        var.put_values(d.as_slice().unwrap(), None, None).unwrap();
    }
}
