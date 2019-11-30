//! Main example function demonstrating core capabilities.

use arc::{
    args, report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
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
    let _form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());

    section("Output");
    report!("Output dir", out_dir.display());

    section("Finished");
}
