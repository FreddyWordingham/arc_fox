//! Main example function showing main capabilities.

use arc::{
    args,
    file::Load,
    form::Setup,
    geom::shape::Aperture,
    init::io_dirs,
    opt::{Light, Spectrum},
    print::term::{section, title},
    report,
    rt::Ray,
    sim::mcrt,
    util::exec,
    world::Universe,
};
use log::info;
use nalgebra::{Point3, Vector3};
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
    let (in_dir, _out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());
    let form = load_form(Some(&in_dir.join(form_path)));
    // let form = load_form(None);
    // form.save(&in_dir.join(form_path));

    section("Setup");
    let _res = form.uni().grid().res();
    let uni = Universe::build(&in_dir, form.uni(), form.num_threads());

    let light = Light::new(
        Box::new(Aperture::new(
            Ray::new(Point3::new(-0.013, 0.0, 0.0), Vector3::x_axis()),
            45.0f64.to_radians(),
        )),
        Spectrum::new_laser(830.0e-9),
        1.0,
    );

    section("Simulation");
    let _mcrt_data = mcrt::run(form.num_threads(), form.total_phot(), &light, &uni);

    section("Post-Processing");

    section("Output");
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
