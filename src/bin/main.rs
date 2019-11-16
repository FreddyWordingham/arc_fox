//! Main example function showing main capabilities.

use arc::{args, init::io_dirs, print::term::title, util::exec};
use std::path::Path;

fn main() {
    title(&exec::name());
    colog::init();

    args!(
        _bin_path: String;
        form_path: String
    );
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);
}
