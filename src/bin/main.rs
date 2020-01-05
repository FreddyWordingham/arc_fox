//! Main testing function.

use arc::{
    args, form, report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use colog;
use std::path::Path;

form!(Parameters, num_threads: usize);

pub fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    args!(_bin_path: String;
        form_name: String);

    let form_name = Path::new(&form_name);
    let (in_dir, out_dir) = io_dirs(None, None);

    report!("Form name", form_name.display());
    report!("Input dir", in_dir.display());
    report!("Output dir", out_dir.display());

    section("Loading");
}
