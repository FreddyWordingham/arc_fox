//! Saveable to file trait.

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

impl<T: Serialize> Saveable for T {
    #[pre(path.is_file(), "Save path is not a file path.")]
    fn save(&self, path: &Path) {
        write(path, to_string(self).expect("Unable to serialise object."))
            .expect("Unable to write serialisation to file.")
    }
}

#[pre(data.shape().iter().all(|x| *x > 0))]
pub fn save_as_netcdf<T: Numeric>(data: &Array3<T>, path: &Path) {
    let mut file = File::create(&path).expect("Unable to create file.");

    let shape = data.shape();

    let dim1_name = "x";
    let dim2_name = "y";
    let dim3_name = "z";
    file.root_mut()
        .add_dimension(dim1_name, shape[0])
        .expect("Unable to add dimension.");
    file.root_mut()
        .add_dimension(dim2_name, shape[1])
        .expect("Unable to add dimension.");
    file.root_mut()
        .add_dimension(dim3_name, shape[2])
        .expect("Unable to add dimension.");

    let var = &mut file
        .root_mut()
        .add_variable::<T>("data", &[dim1_name, dim2_name, dim3_name])
        .expect("Unable to add datacube entry.");
    var.put_values(
        data.as_slice().expect("Unable to write datacube values."),
        None,
        None,
    )
    .expect("Unable to store datacube values.");
}
