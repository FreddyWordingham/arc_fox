//! Rainbow ray-tracing example binary.

use arc::{
    args,
    file::io::Load,
    form, report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use colog;
use log::info;
use std::path::{Path, PathBuf};

form!(Parameters, num_phot: f64);

fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());
    report!("parameters path", params_path.display());

    section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    section("Manifest");
    let num_phot = params.num_phot as u64;
    report!(num_phot);

    section("Simulation");
    info!("Tracing...");
    let _ccd = simulation(num_phot);
    info!("Tracing complete.");

    section("Output");
    info!("Saving...");
    info!("Saving complete.");
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn prelude(params_path: &Path) -> Parameters {
    Parameters::load(&params_path)
}

fn simulation(_num_phot: u64) -> () {}
