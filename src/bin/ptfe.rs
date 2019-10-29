//! PTFE investigation.

use arc::{
    file::Loadable,
    file::Saveable,
    form::input::Ptfe as PtfeForm,
    geom::Ray,
    phy::Photon,
    util::{get_args, title},
};
use log::info;
use std::path::Path;

fn main() {
    // Title.
    title("PTFE");
    colog::init();

    // Start up.
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let input_file_path = Path::new(&args[1]);

    // Load manifest.
    info!("Loading input form: {}", input_file_path.display());
    // let form = PtfeForm::example();
    let form = PtfeForm::load(input_file_path);

    let dir = form.dir().manifest();
    info!("Directory setup:\n{}", dir);

    let dom = form.dom().manifest();
    info!("Domain setup:\n{}", dom);

    let num_phot = form.num_phot();
    info!("Number of photons: {}", num_phot);
    let emission_pos = form.emission_pos();
    info!(
        "Emission position: [{}\t{}\t{}]",
        emission_pos.x, emission_pos.y, emission_pos.z
    );
    let emission_dir = form.emission_dir();
    info!(
        "Emission direction: [{}\t{}\t{}]",
        emission_dir.x, emission_dir.y, emission_dir.z
    );
    let emission_wavelength = form.emission_wavelength();
    info!("Emission wavelength: {}", emission_wavelength);

    // Simulation.
    let bar = arc::util::progress::bar(num_phot as u64);
    for _ in 0..num_phot {
        bar.inc(1);

        let mut phot = Photon::new(Ray::new(emission_pos, emission_dir), emission_wavelength);

        while dom.boundary().contained(phot.ray().origin()) {}
    }

    // Output.
    form.save(&dir.out().join("last_run.json"));
}
