//! MCRT test binary.

use arc::{args, init::io_dirs, print, report, util::bin_name};

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input directory");

    print::section("Initialisation");

    print::section("Simulation");

    print::section("Post-Processing");

    print::section("Output");
    report!(out_dir.display(), "Output directory");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
