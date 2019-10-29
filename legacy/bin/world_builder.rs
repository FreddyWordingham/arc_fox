//! World building example.

use arc::{
    file::Loadable,
    // file::Saveable,
    form::input::WorldBuilder as WorldBuilderForm,
    phy::ThreeDimensional,
    util::{get_args, title},
};
use log::info;
use ndarray::Array3;
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

    let surf_map = form.surf_map().manifest(dir.meshes(), &mat_map);
    info!("Surface map:\n{}", surf_map);

    // Setup.
    let layout = dom.layout();
    let _boundaries = Array3::from_elem(layout.as_array(), 0);
    let bar = arc::util::progress::bar(layout.total() as u64);

    for xi in 0..layout.x() {
        for yi in 0..layout.x() {
            for zi in 0..layout.x() {
                bar.inc(1);
                let _index = [xi, yi, zi];
            }
        }
    }
    bar.finish();

    // Output.
    // form.save(&dir.out().join("example.json"));
    // boundaries.save(&dir.out().join("boundaries.nc"));
}
