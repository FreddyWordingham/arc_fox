//! World building example.

use arc::{
    file::Loadable,
    file::Saveable,
    form::input::WorldBuilder as WorldBuilderForm,
    util::{get_args, title},
};
use log::info;
use std::path::Path;

fn main() {
    // Title.
    title("World Builder");
    colog::init();

    // Start up.
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let input_file_path = Path::new(&args[1]);

    // Load manifest.
    info!("Loading input form: {}", input_file_path.display());
    // let form = WorldBuilderForm::example();
    let form = WorldBuilderForm::load(input_file_path);

    let dir = form.dir().manifest();
    info!("Directory setup:\n{}", dir);

    let dom = form.dom().manifest();
    info!("Domain setup:\n{}", dom);

    let mat_map = form.mat_map().manifest(dir.mats());
    info!("Material map:\n{}", mat_map);

    let _surf_map = form.surf_map().manifest(dir.meshes(), &mat_map);

    // Output.
    form.save(&dir.out().join("example.json"));
}
