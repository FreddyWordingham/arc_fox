//! Main testing binary.

use arc::{
    args,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use attr_mac::form;
use colog;
use std::path::Path;

#[form]
struct Parameters {
    num_threads: usize,
}

pub fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    args!(_bin_path: String;
        form_name: String);

    let form_name = Path::new(&form_name);
    let (in_dir, _out_dir) = io_dirs(None, None);
    let _form_path = &in_dir.join(form_name);
}
