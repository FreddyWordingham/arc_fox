//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::args;
fn main() {
    args!(_bin_path: String);

    println!("Cwd: {}", arc::dir::arc().display());
    println!("res: {}", arc::dir::res::root().display());
    println!("materials: {}", arc::dir::res::materials().display());
    println!("meshes: {}", arc::dir::res::meshes().display());
    println!("reactions: {}", arc::dir::res::reactions().display());
    println!("species: {}", arc::dir::res::species().display());
}
