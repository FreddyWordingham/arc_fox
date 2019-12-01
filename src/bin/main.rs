//! Main example function demonstrating core capabilities.

use arc::{
    args,
    file::io::Load,
    form, report,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::parts::{
        interfaces, interfaces_builder, materials, materials_builder, meshes_builder,
        reactions_builder, species, species_builder,
    },
};
use log::info;
use std::path::Path;

form!(Parameters,
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
    let reactions = reactions_builder::load(&in_dir.join("reactions"), &form.reactions);
    let interfaces = interfaces_builder::load(&in_dir.join("interfaces"), &form.interfaces);
    let meshes = meshes_builder::load(&in_dir.join("meshes"), &interfaces);
    let materials = materials_builder::load(&in_dir.join("materials"), &interfaces);
    let species = species_builder::load(&in_dir.join("species"), &reactions, &materials);

    section("Building");
    let _species = species::build(species);
    let materials = materials::build(materials);
    let _interfaces = interfaces::build(interfaces, &meshes, &materials);

    section("Output");
    report!("Output dir", out_dir.display());

    section("Finished");
}
