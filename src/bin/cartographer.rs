//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{dir::init, print, report, util::start_up};
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _cwd, _out) = start_up();
}

fn title() {
    print::title("CARTOGRAPHER");
    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = start_up::get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let cwd = init::input_dir("cartographer");
    report!(cwd.display(), "input directory");

    let out = init::output_dir("cartographer");
    report!(out.display(), "output directory");

    (args, cwd, out)
}
