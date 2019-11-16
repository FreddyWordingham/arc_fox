//! Main example function showing main capabilities.

use arc::{
    args,
    file::Load,
    form::Setup,
    init::io_dirs,
    print::term::{section, title},
    report,
    util::exec,
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
    info!("Loading input form... {}", form_path.display());
    let form = Setup::load(form_path);

    section("Setup");

    section("Output");
    report!("Output dir", out_dir.display());
}
