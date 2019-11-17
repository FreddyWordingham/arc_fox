//! Main example function showing main capabilities.

use arc::{
    args,
    file::{Load, Save},
    form::Setup,
    init::io_dirs,
    print::term::{section, title},
    report,
    util::exec,
    world::Universe,
};
use log::info;
use std::path::Path;

fn main() {
    title(&exec::name());
    colog::init();

    section("Initialisation");
    args!(
        _bin_path: String;
        form_path: String
    );
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());
    // let form = load_form(Some(&in_dir.join(form_path)));
    let form = load_form(None);

    section("Setup");
    let uni = Universe::build(form.uni());

    section("Output");
    report!("Output dir", out_dir.display());
    info!("Saving copy of input form.");
    form.save(&in_dir.join(form_path));
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
