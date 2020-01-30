//! Main function.

use arc::{
    args,
    file::Save,
    report,
    util::{banner, exec, io_dirs},
};
use colog;
use std::path::PathBuf;

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("initialisation");
    let (in_dir, out_dir) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    let obj = arc::uni::Material::new(
        arc::phys::Optics::new(
            arc::math::Formula::Constant { c: 1.34 },
            arc::math::Formula::Constant { c: 45.0e-3 },
            arc::math::Formula::Constant { c: 0.833e-3 },
            arc::math::Formula::Constant { c: 0.0 },
            arc::math::Formula::Constant { c: 0.8 },
        ),
        Some(8.90e-4),
        Some(1.0),
    );
    obj.save(&out_dir.join("material.json"));
}

fn initialisation() -> (PathBuf, PathBuf) {
    args!(_bin_path: String);

    let (in_dir, out_dir) = io_dirs(None, None);

    (in_dir, out_dir)
}
