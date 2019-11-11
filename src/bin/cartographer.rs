//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args, geom::Aabb, index::Resolution, init::io_dirs, print, report, util::bin_name,
    world::Universe,
};
use nalgebra::{Point3, Similarity3, Translation3, UnitQuaternion, Vector3};

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Setup");

    print::section("Initialisation");
    let n = 25;
    let uni = Universe::new(
        Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0)),
        Resolution::new(n, n, n),
        vec![
            ("torus", "torus", None, "fog", "air"),
            (
                "upper-plane",
                "plane",
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, 0.75),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    1.0,
                )),
                "air",
                "fog",
            ),
            (
                "lower-plane",
                "plane",
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, -0.75),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    1.0,
                )),
                "air",
                "fog",
            ),
        ],
    );

    print::section("Simulation");

    print::section("Post-Processing");

    print::section("Output");
    report!(out_dir.display(), "Output dir");

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
