//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{dir::init, print, report, util::start_up};
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _input, _output) = start_up();
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

    let input = init::input_dir("cartographer");
    report!(input.display(), "input directory");

    let output = init::output_dir("cartographer");
    report!(output.display(), "output directory");

    (args, input, output)
}
