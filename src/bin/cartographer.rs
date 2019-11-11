//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args,
    file::Saveable,
    geom::Aabb,
    index::Resolution,
    init::io_dirs,
    print, report,
    util::bin_name,
    world::{Identity, Universe},
};
use nalgebra::{Point3, Similarity3, Translation3, UnitQuaternion, Vector3};
use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Setup");

    print::section("Initialisation");
    let n = 51;
    let l = 2.0;
    let res = Resolution::new(n, n, n);
    let uni = Universe::new(
        Aabb::new_centred(&Point3::origin(), &Vector3::new(l, l, l)),
        res.clone(),
        vec![
            ("torus", "torus", None, "fog", "air"),
            // (
            //     "upper-plane",
            //     "plane",
            //     Some(Similarity3::from_parts(
            //         Translation3::new(0.0, 0.0, 0.75),
            //         UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            //         1.0,
            //     )),
            //     "air",
            //     "fog",
            // ),
            // (
            //     "lower-plane",
            //     "plane",
            //     Some(Similarity3::from_parts(
            //         Translation3::new(0.0, 0.0, -0.75),
            //         UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            //         1.0,
            //     )),
            //     "fog",
            //     "air",
            // ),
        ],
    );

    print::section("Simulation");

    print::section("Post-Processing");
    let mut intersections = Vec::with_capacity(res.total());
    let mut vals = Vec::with_capacity(res.total());
    for index in res.iter() {
        let cell = &uni.grid().cells()[index.arr];

        if cell.is_empty() {
            intersections.push(0.0);
        } else {
            intersections.push(1.0);
        }

        match cell.mat().id() {
            "air" => {
                vals.push(1.0);
            }
            "fog" => {
                vals.push(2.0);
            }
            _ => unreachable!("Can't get here..."),
        }
    }
    let map = Array3::from_shape_vec(res.arr, vals).unwrap();
    let surf = Array3::from_shape_vec(res.arr, intersections).unwrap();

    print::section("Output");
    report!(out_dir.display(), "Output dir");
    map.save(&out_dir.join("map.nc"));
    surf.save(&out_dir.join("surf.nc"));

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
