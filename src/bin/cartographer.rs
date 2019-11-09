//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{args, init::io_dirs};
fn main() {
    args!(_bin_path: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    println!("Input  : {}", in_dir.display());
    println!("Output : {}", out_dir.display());
}
