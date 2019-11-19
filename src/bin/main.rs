//! Main example function showing main capabilities.

use arc::{
    args,
    file::{Load, Save},
    form::Setup,
    geom::shape::Aperture,
    init::io_dirs,
    opt::{Light, Spectrum},
    print::term::{section, title},
    report,
    rt::Ray,
    sim::{evolve, mcrt},
    util::exec,
    world::{map::index_of_key, Universe},
};
use log::info;
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array3;
use std::path::Path;

fn main() {
    title(&exec::name());
    colog::init();

    section("Initialisation");
    args!(
        _bin_path: String;
        form_path: String
    );
    let _form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());
    let form = load_form(Some(&in_dir.join(form_path)));
    // let form = load_form(None);
    // form.save(&in_dir.join(form_path));

    section("Setup");
    let res = form.uni().grid().res();
    let uni = Universe::build(&in_dir, form.uni());

    let light = Light::new(
        Box::new(Aperture::new(
            Ray::new(
                Point3::new(0.0, 0.0, 7.5e-3),
                Unit::new_normalize(Vector3::new(0.01, 0.01, -1.0)),
            ),
            20.0f64.to_radians(),
        )),
        Spectrum::new_laser(630.0e-9),
        1.0,
    );

    section("Simulation");
    // let pre_state_ = evolve::run(form.num_threads(), 60.0, 15.0, &uni);
    let mcrt_data = mcrt::run(form.num_threads(), form.total_phot(), &light, &uni);

    section("Post-Processing");
    info!("Creating concentration data cube.");
    let mut concs = Vec::with_capacity(res.total());
    for cell in uni.grid().cells().iter() {
        concs.push(cell.concs()[index_of_key(uni.mol_map(), &"ala".to_string())]);
    }
    let concs = Array3::from_shape_vec(res.arr().clone(), concs).unwrap();

    info!("Creating scattering data cube.");
    let vol = uni.grid().cells()[[0, 0, 0]].aabb().vol();
    let mut scats = Vec::with_capacity(res.total());
    let mut total_scats = 0.0;
    for rec in mcrt_data.recs.iter() {
        let x = rec.scatters();
        total_scats += x;
        scats.push(x / vol);
    }
    let scats = Array3::from_shape_vec(res.arr().clone(), scats).unwrap();
    report!("Total scatterings: {}", total_scats);

    info!("Creating travel distance data cube.");
    let vol = uni.grid().cells()[[0, 0, 0]].aabb().vol();
    let mut dist_travelled = Vec::with_capacity(res.total());
    let mut total_dist_travelled = 0.0;
    for rec in mcrt_data.recs.iter() {
        let x = rec.scatters();
        total_dist_travelled += x;
        dist_travelled.push(x / vol);
    }
    let dist_travelled = Array3::from_shape_vec(res.arr().clone(), dist_travelled).unwrap();
    report!("Total distance travelled: {}m", total_dist_travelled);

    section("Output");
    report!("Output dir", out_dir.display());
    info!("Saving concentration datacube.");
    concs.save(&out_dir.join("ala.nc"));

    info!("Saving scattering datacube.");
    scats.save(&out_dir.join("scat.nc"));

    info!("Saving distance travelled datacube.");
    dist_travelled.save(&out_dir.join("dist_travelled.nc"));
}

fn load_form(path: Option<&Path>) -> Setup {
    if let Some(path) = path {
        report!("Loading setup from file", path.display());
        Setup::load(path)
    } else {
        info!("Using example setup.");
        Setup::example()
    }
}
