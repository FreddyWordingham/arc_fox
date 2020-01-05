//! Main testing function.

use arc::{args, form, report, util::dirs::init::io_dirs};
use colog;
use log::info;
use std::path::Path;

form!(Parameters, num_threads: usize);

pub fn main() {
    colog::init();

    info!("Hello world!");

    args!(_bin_path: String;
        form_path: String);

    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    info!("Loading form from {}", form_path.display());
    report!("Input dir", in_dir.display());
    report!("Output dir", out_dir.display());
}
