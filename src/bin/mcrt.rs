//! MCRT test binary.

use arc::{
    args,
    file::Saveable,
    form::Mcrt,
    geom::Aabb,
    init::io_dirs,
    print, report,
    sim::mcrt,
    util::bin_name,
    world::{Light, Universe},
};
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
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
    report!("X-width", dom.widths().x, "m");
    report!("Y-width", dom.widths().y, "m");
    report!("Z-width", dom.widths().z, "m");
    report!("Volume", dom.vol(), "m^3");

    let mut uni = Universe::new(&in_dir, dom, res, form.ents());

    let light = Light::new(
        Box::new((Point3::origin(), Vector3::x_axis(), 45.0f64.to_radians())),
        630.0e-9, // [m]
        1.0,      // [J/s]
    );

    print::section("Simulation");
    let mcrt_data = mcrt::run(form.num_threads(), form.total_phot(), &light, &uni);
    uni.add_archive(mcrt_data);

    // print::section("Post-Processing");
    // info!("Creating record cube.");
    // let recs = uni.grid().cells().map(|c| c.rec());

    // info!("Creating emission data cube.");
    // let mut emissions = Vec::with_capacity(uni.grid().res().total());
    // let mut scatters = Vec::with_capacity(uni.grid().res().total());
    // let mut absorptions = Vec::with_capacity(uni.grid().res().total());
    // let mut dist_travelled = Vec::with_capacity(uni.grid().res().total());
    // for rec in recs.iter() {
    //     emissions.push(rec.emissions());
    //     scatters.push(rec.scatters());
    //     absorptions.push(rec.absorptions());
    //     dist_travelled.push(rec.absorptions());
    // }
    // let emissions = Array3::from_shape_vec(uni.grid().res().arr, emissions).unwrap();
    // let scatters = Array3::from_shape_vec(uni.grid().res().arr, scatters).unwrap();
    // let absorptions = Array3::from_shape_vec(uni.grid().res().arr, absorptions).unwrap();
    // let dist_travelled = Array3::from_shape_vec(uni.grid().res().arr, dist_travelled).unwrap();

    // print::section("Output");
    // report!("Output dir", out_dir.display());
    // emissions.save(&out_dir.join("emissions.nc"));
    // scatters.save(&out_dir.join("scatters.nc"));
    // absorptions.save(&out_dir.join("absorptions.nc"));
    // dist_travelled.save(&out_dir.join("dist_travelled.nc"));

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
