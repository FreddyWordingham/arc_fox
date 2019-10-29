//! PTFE investigation.

use arc::{
    file::Loadable,
    file::Saveable,
    form::input::Ptfe as PtfeForm,
    util::{get_args, title},
};
use log::info;
use std::path::Path;

fn main() {
    // Title.
    title("PTFE");
    colog::init();

    // Start up.
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let input_file_path = Path::new(&args[1]);

    // Load manifest.
    info!("Loading input form: {}", input_file_path.display());
    let form = PtfeForm::example();

    let dir = form.dir().manifest();
    info!("Directory setup:\n{}", dir);

    // Output.
    form.save(&dir.out().join("last_run.json"));
}
