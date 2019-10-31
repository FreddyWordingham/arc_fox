//! PTFE investigation.

use arc::{
    dir::init,
    file::{Loadable, Saveable},
    math::Formula,
    phys::Material,
    report,
    util::{print, start_up::get_args},
};
use log::info;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, cwd, _out) = start_up();
    let () = init();

    let mat = Material::new(Formula::Const(3.14159));
    mat.save(&cwd.join("test.json"));

    report!(arc::dir::res::root().display());
    report!(arc::dir::res::meshes().display());
    report!(arc::dir::res::mats().display());
    report!(arc::dir::res::species().display());
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
