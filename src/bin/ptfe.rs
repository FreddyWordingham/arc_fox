//! PTFE investigation.

use arc::{
    dir::init,
    file::{Loadable, Saveable},
    math::Formula,
    phys::Material,
    report,
    util::{print, start_up::get_args, Range},
};
use log::info;
use std::{collections::HashMap, path::PathBuf};

fn main() {
    title();
    let (_args, _cwd, _out) = start_up();
    let () = init();

    let mat = Material::new(
        Range::positive(),
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

    let mat_dir = arc::dir::res::mats();

    let mats = vec!["intralipid", "ptfe"];

    let mut mat_map = HashMap::new();
    for name in mats {
        report!("Loading material: {}", name);
        mat_map.insert(
            name,
            Material::load(&mat_dir.join(format!("{}.json", name))),
        );
    }
}
