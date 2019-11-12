//! Monte-Carlo Radiative Transfer binary.
//! Sets up and runs an MCRT simulation.

use arc::{
    args,
    dim::Cartesian::{X, Y, Z},
    file::Loadable,
    file::Saveable,
    form::Setup,
    geom::Aabb,
    index::Resolution,
    init::io_dirs,
    print, report,
    util::bin_name,
    world::Light,
    world::{Identity, Universe},
};
use nalgebra::{Point3, Vector3};
use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");
    let setup = Setup::load(&in_dir.join("setup.json"));
    // let setup = Setup::example();
    // setup.save(&in_dir.join("setup.json"));

    print::section("Initialisation");
    let uni = Universe::new(
        Aabb::new_centred(
            &Point3::origin(),
            &Vector3::new(
                setup.half_widths[X as usize],
                setup.half_widths[Y as usize],
                setup.half_widths[Z as usize],
            ),
        ),
        Resolution::new(
            setup.resolution[X as usize],
            setup.resolution[Y as usize],
            setup.resolution[Z as usize],
        ),
        setup.ent_info,
    );

    let light = Light::new(
        Box::new((Point3::origin(), Vector3::x_axis(), 45.0f64.to_radians())),
        630.0e-9, // [m]
        1.0,      // [J/s]
    );

    print::section("Post-Processing");

    print::section("Output");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
