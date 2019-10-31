//! PTFE investigation.

use arc::{
    report,
    util::{
        print::{section, title},
        start_up::get_args,
    },
};
use log::info;

fn main() {
    // Title.
    title("PTFE");
    colog::init();

    // Start up.
    section("Start Up");
    let args = get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    // Initialisation.
    section("Initialising");
}
