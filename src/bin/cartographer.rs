//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{args, init::io_dirs, print, report, util::bin_name};

fn main() {
    title();
    args!(_bin_path: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    report!(in_dir.display(), "Input dir");
    report!(out_dir.display(), "Output dir");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
