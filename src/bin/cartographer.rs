//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    dir::init,
    geom::Shape,
    print, report,
    util::start_up,
    world::{load_ent_map, load_mat_map},
};
use nalgebra::Vector3;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _input, _output) = start_up();

    print::section("Initialisation");
    let mat_map = load_mat_map(
        &arc::dir::res::mats(),
        &vec!["air".to_string(), "fog".to_string()],
    );
    let _ent_map = load_ent_map(&vec![
        (
            Shape::new_plane(0.3, -Vector3::x_axis()),
            &mat_map["air"],
            &mat_map["fog"],
        ),
        (
            Shape::new_plane(0.5, -Vector3::x_axis()),
            &mat_map["fog"],
            &mat_map["air"],
        ),
    ]);
}

fn title() {
    print::title("CARTOGRAPHER");
    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = start_up::get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let input = init::input_dir("cartographer");
    report!(input.display(), "input directory");

    let output = init::output_dir("cartographer");
    report!(output.display(), "output directory");

    (args, input, output)
}
