//! MCRT test binary.

use arc::{
    args, file::Saveable, form::Mcrt, geom::Aabb, init::io_dirs, print, report, util::bin_name,
};
use log::info;
use nalgebra::Point3;
use std::path::Path;

fn main() {
    title();
    args!(_bin_path: String, form_path: String);
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!("Input directory", in_dir.display());
    info!("Loading form: {}", form_path.display());
    let form = Mcrt::example();
    form.save(&in_dir.join("example.json"));

    print::section("Initialisation");
    let res = form.res();
    report!("Grid resolution", res);
    report!("Total cells", res.total());

    let dom = Aabb::new_centred(&Point3::origin(), form.half_widths());
    report!("X-width", dom.half_widths().x);
    report!("Y-width", dom.half_widths().y);
    report!("Z-width", dom.half_widths().z);
    report!("Volume", dom.vol());

    print::section("Simulation");

    print::section("Post-Processing");

    print::section("Output");
    report!("Output directory", out_dir.display());

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
