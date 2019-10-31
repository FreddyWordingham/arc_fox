//! Saveable to file trait.

use hdf5::H5Type;
use ndarray::Array3;
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
pub fn save_as_hdf5<T: H5Type>(data: Vec<(&'static str, &Array3<T>)>, path: &Path) {
    let file = hdf5::File::open(path, "w").expect("Unable to create file.");

    for (name, d) in data {
        let shape = d.shape();

        let dataset = file
            .new_dataset::<T>()
            .create(name, (shape[0], shape[1], shape[2]))
            .expect("Unable to create data set entry.");
        dataset
            .write(d.as_slice().unwrap())
            .expect("Unable to write data set.");
    }
}
