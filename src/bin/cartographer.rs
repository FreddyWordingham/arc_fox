//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args, file::save_as_netcdf, index::Resolution, init::io_dirs, print, report, util::bin_name,
};
use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Setup");
    let res = Resolution::new(2, 3, 4);

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Simulation");

    print::section("Post-Processing");
    let mut intersection: Vec<f64> = Vec::with_capacity(res.total());
    for index in res.iter() {
        println!("{}\t{}\t{}", index.x(), index.y(), index.z());
        intersection.push(1.0);
    }

    let intersection =
        Array3::from_shape_vec(*res.arr(), intersection).expect("Unable to form vec into array3.");

    print::section("Output");
    report!(out_dir.display(), "Output dir");
    save_as_netcdf(&intersection, &out_dir.join("intersection.nc"));

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
