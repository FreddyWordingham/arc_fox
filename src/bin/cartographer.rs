//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

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
    // let setup = Setup::load(&in_dir.join("setup.json"));
    let setup = Setup::example();
    setup.save(&in_dir.join("setup.json"));

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

    // let light = Light::new(
    //     Box::new(
    //         (Point3::origin(), Vector3::x_axis(), 45.0f64.to_radians())
    //     ),
    //     630.0e-9, // [m]
    //     1.0 // [J/s]
    //  );

    print::section("Mapping");
    let res = uni.grid().res();
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
