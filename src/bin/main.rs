//! Main example function demonstrating core capabilities.

use ndarray::Array3;

fn main() {
    println!("Hello World!");

    let id: Index = [10, 10, 10];

    let _arr = Array3::from_shape_vec(id, vec![0; 100]);
}
//
type Index = [usize; 3];
