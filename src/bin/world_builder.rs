//! World building example.

use arc::{
    // file::Loadable,
    file::Saveable,
    form::{input::WorldBuilder as WorldBuilderForm, Manifestable},
    util::{get_args, title},
};
use log::info;
use std::path::Path;

fn main() {
    // Title.
    title("World Builder");

    // Start up.
    // let (_cwd, _out_dir) = start_up(&Path::new("cwd"), &Path::new("out"));
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let input_file_path = Path::new(&args[1]);

    // Load manifest.
    info!("Loading input form: {}", input_file_path.display());
    // let form = WorldBuilderForm::load(input_file_path);
    let form = WorldBuilderForm::example();
    let dir = form.dir().manifest();

    // Output.
    form.save(&dir.out().join("example.json"));
}
