//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    dir::init,
    dom::Aabb,
    file::save_as_netcdf,
    geom::Shape,
    index::Layout,
    print,
    proto::Entity as ProtoEntity,
    report,
    rng::Distribution,
    rt::Emitter,
    util::start_up,
    world::{Light, Universe},
};
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _input, output) = start_up();

    print::section("Initialisation");
    let uni = Universe::new(
        Layout::new(17, 17, 17),
        Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0)),
        vec![
            ProtoEntity::new(
                Shape::new_plane(Point3::new(0.3, 0.0, 0.0), -Vector3::x_axis()),
                "air",
                "fog",
            ),
            ProtoEntity::new(
                Shape::new_plane(Point3::new(0.5, 0.0, 0.0), -Vector3::x_axis()),
                "fog",
                "air",
            ),
        ],
    );

    print::section("Simulation");
    let _light_map = uni.mcrt(
        4,
        &Light::new(
            Emitter::new_point(Point3::origin()),
            Distribution::new_const(630.0e-9),
            1.0,
        ),
    );

    let grid = uni.grid();
    let layout = grid.layout();
    let mut scat_coeffs = Vec::with_capacity(layout.total_indices());
    for xi in 0..layout.x() {
        for yi in 0..layout.y() {
            for zi in 0..layout.z() {
                let index = [xi, yi, zi];
                let cell = &grid.cells()[index];
                let mat = cell.mat();

                scat_coeffs.push(mat.scat_coeff(700.0e-9));
            }
        }
    }

    print::section("Post-processing");
    let scat_coeffs = Array3::from_shape_vec(*layout.nis(), scat_coeffs).unwrap();

    print::section("Output");
    save_as_netcdf(&output.join("data.nc"), vec![("scat_coeff", &scat_coeffs)]);

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
