//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    dir::init,
    dom::{Aabb, Grid},
    geom::Shape,
    index::Layout,
    print, report,
    util::start_up,
    world::{load_ent_map, load_mat_map},
};
use nalgebra::{Point3, Vector3};
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _input, _output) = start_up();

    print::section("Initialisation");
    let mat_map = load_mat_map(
        &arc::dir::res::mats(),
        &vec!["air".to_string(), "fog".to_string()],
    );
    let _ent_map = load_ent_map(vec![
        (
            "block_start".to_string(),
            Shape::new_plane(Point3::new(0.3, 0.0, 0.0), -Vector3::x_axis()),
            &mat_map["air"],
            &mat_map["fog"],
        ),
        (
            "block_end".to_string(),
            Shape::new_plane(Point3::new(0.5, 0.0, 0.0), -Vector3::x_axis()),
            &mat_map["fog"],
            &mat_map["air"],
        ),
    ]);
    let grid = Grid::new(
        Layout::new(17, 17, 17),
        Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0)),
    );
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
