//! Save trait.

use crate::list::dimension::Cartesian::{X, Y, Z};
use contracts::pre;
use ndarray::Array3;
use netcdf::{variable::Numeric, File};
use serde::Serialize;
use serde_json::to_string;
use std::{fmt::Debug, fs::write, path::Path};

/// Save trait implementation.
/// Types implementing this trait can be saved to a file.
pub trait Save: Debug {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the object in json format if it implements the Serialize trait.
pub fn as_json<T: Serialize>(obj: &T, path: &Path) {
    write(path, to_string(obj).expect("Unable to serialise object."))
        .expect("Unable to write json file.")
}

impl<T: Debug + Numeric> Save for Array3<T> {
    #[pre(self.len() > 0)]
    fn save(&self, path: &Path) {
        let mut file = File::create(&path).expect("Unable to create file.");

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

impl<T: Debug + Numeric> Save for Vec<(&str, Array3<T>)> {
    #[pre(self.len() > 0)]
    #[pre(self.iter().all(|(name, arr)| !name.is_empty() && arr.len() > 0))]
    fn save(&self, path: &Path) {
        let mut file = File::create(&path).expect("Unable to create file.");

        let shape = self[0].1.shape();

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

        for (name, arr) in self.iter() {
            if arr.shape() != shape {
                panic!("Unable to save datacube pack, shapes do not match.");
            }

            let var = &mut file
                .root_mut()
                .add_variable::<T>(name, &[dim1_name, dim2_name, dim3_name])
                .expect("Unable to add datacube entry.");
            var.put_values(
                arr.as_slice().expect("Unable to write datacube values."),
                None,
                None,
            )
            .expect("Unable to store datacube values.");
        }
    }
}
