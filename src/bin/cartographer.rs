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

    print::section("Initialisation");
    let n = 51;
    let l = 2.0;
    let res = Resolution::new(n, n, n);
    let uni = Universe::new(
        Aabb::new_centred(&Point3::origin(), &Vector3::new(l, l, l)),
        res.clone(),
        vec![
            (
                "torus",
                "torus",
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, 0.0),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    0.75,
                )),
                "thick_fog",
                "air",
            ),
            (
                "upper-plane",
                "plane",
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, 1.5),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    2.5,
                )),
                "air",
                "fog",
            ),
            (
                "lower-plane",
                "plane",
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, -1.5),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    2.5,
                )),
                "fog",
                "air",
            ),
        ],
    );

    // let light = Light::new(
    //     Box::new(
    //         (Point3::origin(), Vector3::x_axis(), 45.0f64.to_radians())
    //     ),
    //     630.0e-9, // [m]
    //     1.0 // [J/s]
    //  );

    print::section("Mapping");
    let mut intersections = Vec::with_capacity(res.total());
    let mut vals = Vec::with_capacity(res.total());
    for index in res.iter() {
        let cell = &uni.grid().cells()[index.arr];

        if cell.is_empty() {
            intersections.push(0.0);
        } else {
            intersections.push(1.0);
        }

        let mut misses = 0;
        for mat in uni.mats().iter() {
            if cell.mat().id() == mat.id() {
                vals.push(misses as f64);
                break;
            }
            misses += 1;
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
