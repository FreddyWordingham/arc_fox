//! Main example function showing main capabilities.

use arc::{
    args,
    base::Resolution,
    dom::ProtoGrid,
    file::{Load, Save},
    form::Parameters,
    geom::shape::Aperture,
    init::io_dirs,
    opt::{Light, Spectrum},
    print::{
        format,
        term::{section, title},
    },
    report,
    rt::Ray,
    sim::{evolve::Statemap, mcrt},
    util::exec,
};
use log::info;
use nalgebra::{Point3, Unit, Vector3};
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
    // let _form = load_form(Some(&in_dir.join(form_path)));
    let form = load_form(None);
    form.save(&in_dir.join(form_path));

    section("Setup");
    let _res = form.grid().res();
    let uni = form.manifest(&in_dir);

    section("Pre-Flight");
    info!("{}", format::universe(&uni));

    let light = Light::new(
        Box::new(Aperture::new(
            Ray::new(
                Point3::new(0.0, 0.0, 7.5e-3),
                Unit::new_normalize(Vector3::new(1.0, 0.01, 0.01)),
            ),
            20.0f64.to_radians(),
        )),
        Spectrum::new_laser(630.0e-9),
        1.0,
    );

    section("Simulation");
    // let _pre_state = evolve::run(form.num_threads(), 60.0, 15.0, &uni);
    let lightmap = mcrt::run(form.num_threads(), 1_000, &light, &uni);
    let states = Statemap::new(uni.grid());

    section("Output");
    info!("Saving lightmap.");
    lightmap.save(&out_dir.join("lightmap.nc"));

    info!("Saving initial states.");
    states
        .mol_concs(uni.mol_map())
        .save(&out_dir.join("concs.nc"));

    section("Finished");
}

fn load_form(path: Option<&Path>) -> Parameters {
    if let Some(path) = path {
        report!("Loading setup from file", path.display());
        Parameters::load(path)
    } else {
        info!("Using example setup.");
        Parameters::new(
            1,
            ProtoGrid::new(Resolution::new(11, 11, 11), Vector3::new(1.0, 1.0, 1.0)),
            vec!["chunk", "wall"],
            Some(vec!["cell_death_mechanism", "ppix_formation"]),
            None,
        )
    }
}
