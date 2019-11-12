//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args,
    file::Loadable,
    form::Setup,
    init::io_dirs,
    print, report,
    sim::mcrt,
    util::bin_name,
    world::{Light, Universe},
};
use nalgebra::{Point3, Vector3};

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, _out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");
    let setup = Setup::load(&in_dir.join("setup.json"));

    print::section("Initialisation");
    let uni = Universe::new_from_setup(setup);
    let light = Light::new(
        Box::new((Point3::origin(), Vector3::x_axis(), 45.0f64.to_radians())),
        630.0e-9, // [m]
        1.0,      // [J/s]
    );

    print::section("Simulation");
    let _mcrt_data = mcrt::run(4, 1_000_000, &light, &uni);

    print::section("Post-Processing");

    print::section("Output");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
