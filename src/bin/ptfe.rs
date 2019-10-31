//! PTFE investigation.

use arc::{
    report,
    util::{
        print::{section, title},
        start_up::{get_args, get_cwd},
    },
};
use log::info;
use std::path::Path;

fn main() {
    title();
    let (args, cwd) = start_up();

    // Initialisation.
    section("Initialising");
}

fn title() {
    title("PTFE");
    colog::init();
}

fn start_up() -> (Vec<String>, Path) {
    section("Start Up");
    let args = get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let cwd = get_cwd();
    report!(cwd);

    (args, cwd)
}
