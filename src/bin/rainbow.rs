//! Rainbow ray-tracing example binary.

use arc::{
    args,
    file::io::{Load, Save},
    form, report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use colog;
use log::info;
use ndarray::Array2;
use proc_mac::{HelloMacro, Noob};
use std::path::{Path, PathBuf};

form!(Parameters,
    num_phot: f64;
    res: (usize, usize)
);

#[derive(Debug, HelloMacro, Noob)]
struct Thing {
    fgh: f64,
    beans: usize,
}

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
    let ccd = simulation(num_phot, params.res);
    info!("Tracing complete.");

    section("Output");
    info!("Saving...");
    ccd.save(&out_dir.join("ccd.nc"));
    info!("Saving complete.");

    println!("THis:\n{:?}", params);

    Thing::hello_macro();
    let t = Thing::new(2.0, 6);
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

fn simulation(_num_phot: u64, res: (usize, usize)) -> Array2<f64> {
    let num_hits = Array2::zeros(res);

    num_hits
}
