//! Main example function showing main capabilities.

use arc::{
    args,
    file::{Load, Save},
    form::Setup,
    init::io_dirs,
    print::term::{section, title},
    report,
    util::exec,
    world::{map::index_of_key, Universe},
};
use log::info;
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

    section("Post-Processing");
    info!("Creating concentration data cube.");
    let mut concs = Vec::with_capacity(res.total());
    for cell in uni.grid().cells().iter() {
        concs.push(cell.concs()[index_of_key(uni.mol_map(), &"ala".to_string())]);
    }
    let concs = Array3::from_shape_vec(res.arr().clone(), concs).unwrap();

    section("Output");
    report!("Output dir", out_dir.display());
    info!("Saving concentration datacube.");
    concs.save(&out_dir.join("ala.nc"));
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
