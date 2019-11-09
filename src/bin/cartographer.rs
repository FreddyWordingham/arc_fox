//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{args, index::Resolution, init::io_dirs, print, report, util::bin_name};
// use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Setup");
    let res = Resolution::new(8, 8, 8);

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Post-Processing");
    let _intersection: Vec<bool> = Vec::with_capacity(res.total());

    print::section("Output");
    report!(out_dir.display(), "Output dir");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
