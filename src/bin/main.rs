//! Main example function demonstrating core capabilities.

use arc::{
    args,
    file::io::{Load, Save},
    form, report,
    sci::math::shape::Aabb,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::{Universe, UniverseBuilder},
};
use log::info;
use nalgebra::{Point3, Vector3};
use std::path::Path;

form!(Parameters,
    num_threads: usize;
    half_widths: Vector3<f64>;
    res: [usize; 3];
    reactions: Vec<String>;
    interfaces: Vec<String>
);

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

    section("Loading");
    report!("Input dir", in_dir.display());
    report!(
        "Loading parameters from file",
        in_dir.join(form_path).display()
    );
    let form = Parameters::load(&in_dir.join(form_path));
    let builder = UniverseBuilder::new(
        Aabb::new_centred(&Point3::origin(), &form.half_widths),
        form.res,
        &in_dir,
        &form.reactions,
        &form.interfaces,
    );

    section("Building");
    let universe = Universe::build(form.num_threads, builder);

    section("Setup");
    arc::util::format::universe(&universe);

    section("Post-Processing");
    let mat = universe.generate_mat_maps();

    section("Output");
    report!("Output dir", out_dir.display());
    mat.save(&out_dir.join("materials.nc"));

    section("Finished");
}
