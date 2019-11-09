//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{args, init::io_dirs, print, report, util::bin_name, index::Resolution};
use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Setup");
    let res = Resolution::new();

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Post-Processing");
    let intersection = Vec::with_capacity(capacity: usize)

    print::section("Output");
    report!(out_dir.display(), "Output dir");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
