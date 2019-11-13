//! MCRT test binary.

use arc::{
    args, file::Saveable, form::Entity as EntityForm, form::Mcrt, geom::Aabb, init::io_dirs, print,
    report, util::bin_name, world::Material,
};
use contracts::{post, pre};
use log::info;
use nalgebra::Point3;
use std::path::Path;

fn main() {
    title();
    args!(_bin_path: String, form_path: String);
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!("Input directory", in_dir.display());
    info!("Loading form: {}", form_path.display());
    let form = Mcrt::example();
    form.save(&in_dir.join("example.json"));

    print::section("Initialisation");
    let res = form.res();
    report!("Grid resolution", res);
    report!("Total cells", res.total());

    let dom = Aabb::new_centred(&Point3::origin(), form.half_widths());
    report!("X-width", dom.widths().x, "m");
    report!("Y-width", dom.widths().y, "m");
    report!("Z-width", dom.widths().z, "m");
    report!("Volume", dom.vol(), "m^3");

    let _mats = load_mats(form.ents());

    // let uni = Universe::

    print::section("Simulation");

    print::section("Post-Processing");

    print::section("Output");
    report!("Output directory", out_dir.display());

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}

fn load_mats(ents: &Vec<EntityForm>) -> Vec<Material> {
    let mats = Vec::new();

    mats
}

#[pre(!ents.is_empty())]
#[post(!ret.is_empty())]
fn get_mat_names(ents: &Vec<EntityForm>) -> Vec<String> {
    let mut mat_names;

    for ent in ents.iter() {
        mat_names.push(ent.in_mat.clone());
        mat_names.push(ent.out_mat.clone());
    }

    mat_names.sort();
    mat_names.dedup();

    mat_names
}
