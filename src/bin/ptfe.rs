//! PTFE investigation.

use arc::{
    report,
    util::{
        print,
        start_up::{get_args, init_cwd, init_out},
    },
};
use log::info;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _cwd, _out) = start_up();
    let () = init();
}

fn title() {
    print::title("PTFE");

    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let cwd = init_cwd("ptfe");
    report!(cwd.display(), "cwd");

    let out = init_out();
    report!(out.display(), "out");

    (args, cwd, out)
}

fn init() {
    print::section("Initialising");
}
