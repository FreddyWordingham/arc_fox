//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    dir::init,
    dom::{Aabb, Grid},
    geom::Shape,
    file::save_as_netcdf,
    index::Layout,
    print, report,
    util::start_up,
    world::{load_ent_map, load_mat_map},
};
use nalgebra::{Point3, Vector3};
use std::path::PathBuf;
use ndarray::Array3;

fn main() {
    title();
    let (_args, _input, output) = start_up();

    print::section("Initialisation");
    let mat_map = load_mat_map(
        &arc::dir::res::mats(),
        &vec!["air".to_string(), "fog".to_string()],
    );
    let ent_map = load_ent_map(vec![
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
        &ent_map,
    );
    let layout = grid.layout();

    print::section("Simulation");
    let mut scat_coeffs = Vec::with_capacity(layout.total_indices());
    for xi in 0..layout.x() {
    for yi in 0..layout.y() {
    for zi in 0..layout.z() {
        let index = [xi, yi, zi];
        let cell = &grid.cells()[index];
        let mat = cell.mat();

        scat_coeffs.push(mat.scat_coeff(700.0e-9));
    }}}

    print::section("Post-processing");
    let scat_coeffs = Array3::from_shape_vec(*layout.nis(), scat_coeffs).unwrap();

    print::section("Output");
    save_as_netcdf(&output.join("data.nc"),vec![("scat_coeff", &scat_coeffs)]);

    print::section("Finished");
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
