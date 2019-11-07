//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    dir::init,
    dom::Aabb,
    file::save_as_netcdf,
    geom::Shape,
    index::Layout3,
    print,
    proto::Entity as ProtoEntity,
    report,
    rng::Distribution,
    rt::Emitter,
    sim,
    util::start_up,
    world::{Light, Universe},
};
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use std::path::PathBuf;

fn main() {
    title();
    let (_args, _input, output) = start_up();

    print::section("Initialisation");
    let mut uni = Universe::new(
        Layout3::new(17, 17, 17),
        Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0)),
        vec![
            ProtoEntity::new(
                Shape::new_plane(Point3::new(0.25, 0.0, 0.0), -Vector3::x_axis()),
                "fog",
                "air",
            ),
            ProtoEntity::new(
                Shape::new_plane(Point3::new(0.5, 0.0, 0.0), -Vector3::x_axis()),
                "air",
                "fog",
            ),
        ],
    );

    print::section("Simulation");
    let light_map = sim::mcrt::run(
        4,
        1_000,
        &Light::new(
            Emitter::new_spotlight(
                Point3::new(-1.0, 0.0, 0.0),
                Vector3::x_axis(),
                45.0f64.to_radians(),
            ),
            Distribution::new_const(630.0e-9),
            1.0,
        ),
        &uni,
    );
    uni.add_archive(light_map);

    print::section("Post-processing");
    info!("Creating record cube.");
    let recs = uni.grid().cells().map(|c| c.rec());

    info!("Creating emission data cube.");
    let mut emissions = Vec::with_capacity(uni.grid().layout().total_indices());
    let mut scatters = Vec::with_capacity(uni.grid().layout().total_indices());
    let mut absorptions = Vec::with_capacity(uni.grid().layout().total_indices());
    for rec in recs.iter() {
        emissions.push(rec.emissions());
        scatters.push(rec.scatters());
        absorptions.push(rec.absorptions());
    }
    let emissions = Array3::from_shape_vec(uni.grid().layout().nis, emissions).unwrap();
    let scatters = Array3::from_shape_vec(uni.grid().layout().nis, scatters).unwrap();
    let absorptions = Array3::from_shape_vec(uni.grid().layout().nis, absorptions).unwrap();

    print::section("Output");
    info!("Saving emissions data.");
    save_as_netcdf(
        &output.join("emissions.nc"),
        vec![
            ("emissions", &emissions),
            ("scatters", &scatters),
            ("absorptions", &absorptions),
        ],
    );

    print::section("Finished");
}

fn title() {
    print::title("CARTOGRAPHER");
    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = start_up::get_args(vec![]);
    for (i, a) in args.iter().enumerate() {
        report!(a, (format!("args[{}]", i)));
    }

    let input = init::input_dir("cartographer");
    report!(input.display(), "input directory");

    let output = init::output_dir("cartographer");
    report!(output.display(), "output directory");

    (args, input, output)
}
