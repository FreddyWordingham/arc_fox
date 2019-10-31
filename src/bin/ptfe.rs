//! PTFE investigation.

use arc::{
    dir::init,
    // file::Loadable,
    file::Saveable,
    math::Formula,
    phys::Material,
    report,
    util::{print, start_up::get_args},
};
use log::info;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _cwd, _out) = start_up();
    let () = init();

    let mat = Material::new(
        Formula::Const(1.0),
        Formula::Const(10.0),
        Formula::Const(0.1),
        Formula::Const(0.0),
        Formula::Const(0.01),
    );
    mat.save(&arc::dir::res::mats().join("test.json"));
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

    let cwd = init::cwd("ptfe");
    report!(cwd.display(), "cwd");

    let out = init::output();
    report!(out.display(), "out");

    (args, cwd, out)
}

fn init() {
    print::section("Initialising");
}
