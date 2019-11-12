//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args,
    file::Loadable,
    file::Saveable,
    form::Setup,
    init::io_dirs,
    print, report,
    util::bin_name,
    world::{Identity, Universe},
};
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
    let uni = Universe::new_from_setup(setup);

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
