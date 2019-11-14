//! Laura's PTFE investigation binary.

use arc::{
    args,
    file::{Loadable, Saveable},
    form::Setup,
    init::io_dirs,
    print, report,
    sim::mcrt,
    util::bin_name,
    world::{Light, Universe},
};
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;

fn main() {
    title();
    args!(_bin_path: String, total_phot: u64, num_threads: usize);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");
    let setup = Setup::load(&in_dir.join("setup.json"));

    print::section("Initialisation");
    let mut uni = Universe::new_from_setup(setup);
    let light = Light::new(
        Box::new((Point3::new(-0.013, 0.0, 0.0), Vector3::x_axis(), 0.1f64.to_radians())),
        630.0e-9, // [m]
        1.0,      // [J/s]
    );

    print::section("Simulation");
    let mcrt_data = mcrt::run(num_threads, total_phot, &light, &uni);
    uni.add_archive(mcrt_data);

    print::section("Post-Processing");
    info!("Creating record cube.");
    let recs = uni.grid().cells().map(|c| c.rec());

    info!("Creating emission data cube.");
    let mut emissions = Vec::with_capacity(uni.grid().res().total());
    let mut scatters = Vec::with_capacity(uni.grid().res().total());
    let mut absorptions = Vec::with_capacity(uni.grid().res().total());
    let mut dist_travelled = Vec::with_capacity(uni.grid().res().total());
    let mut tot_raman = Vec::with_capacity(uni.grid().res().total());
    let mut det_raman = Vec::with_capacity(uni.grid().res().total());
    for rec in recs.iter() {
        emissions.push(rec.emissions());
        scatters.push(rec.scatters());
        absorptions.push(rec.absorptions());
        dist_travelled.push(rec.dist_travelled());
        tot_raman.push(rec.tot_raman());
        det_raman.push(rec.det_raman());
    }
    let emissions = Array3::from_shape_vec(uni.grid().res().arr, emissions).unwrap();
    let scatters = Array3::from_shape_vec(uni.grid().res().arr, scatters).unwrap();
    let absorptions = Array3::from_shape_vec(uni.grid().res().arr, absorptions).unwrap();
    let dist_travelled = Array3::from_shape_vec(uni.grid().res().arr, dist_travelled).unwrap();
    let tot_raman = Array3::from_shape_vec(uni.grid().res().arr, tot_raman).unwrap();
    let det_raman = Array3::from_shape_vec(uni.grid().res().arr, det_raman).unwrap();

    print::section("Output");
    report!(out_dir.display(), "Output dir");
    emissions.save(&out_dir.join("emissions.nc"));
    scatters.save(&out_dir.join("scatters.nc"));
    absorptions.save(&out_dir.join("absorptions.nc"));
    dist_travelled.save(&out_dir.join("dist_travelled.nc"));
    tot_raman.save(&out_dir.join("tot_raman.nc"));
    det_raman.save(&out_dir.join("det_raman.nc"));

    info!("Total Raman photons: {}", tot_raman.sum());
    info!("Detected Raman photons: {}", det_raman.sum());

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}
