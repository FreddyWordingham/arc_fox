//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::args;
fn main() {
    args!(path: String, num_phot: usize);

    println!("{}", path);
    println!("{}", num_phot);
}
