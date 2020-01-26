//! Cartographer testing binary.

use arc::{
    args,
    file::io::Load,
    ord::{dom::Grid, interfaces, reactions, Set},
    report,
    sci::{
        chem::{Reaction, Species},
        math::geom::shape::{Aabb, Mesh},
        phys::{Interface, Material},
    },
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
};
use attr_mac::form;
use colog;
use log::info;
use nalgebra::Point3;
use std::path::{Path, PathBuf};

#[form]
struct Parameters {
    num_threads: usize,
    reactions: Vec<String>,
    interfaces: Vec<String>,
    half_widths: [f64; 3],
    shape: [usize; 3],
}

pub fn main() {
    colog::init();
    title(&exec::name());

    section("Initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    section("Building");
    let reactions = Set::<Reaction>::load(
        &in_dir.join("reactions"),
        params.reactions.as_slice(),
        "json",
    );

    let species = reactions::req_species(&reactions);
    let species = Set::<Species>::load(&in_dir.join("species"), &species, "json");

    let interfaces = Set::<Interface>::load(
        &in_dir.join("interfaces"),
        params.interfaces.as_slice(),
        "json",
    );

    let materials = interfaces::req_materials(&interfaces);
    let materials = Set::<Material>::load(&in_dir.join("materials"), &materials, "json");

    let meshes = interfaces::req_meshes(&interfaces);
    let meshes = Set::<Mesh>::load(&in_dir.join("meshes"), &meshes, "obj");

    let half_widths = Point3::new(
        params.half_widths[0],
        params.half_widths[1],
        params.half_widths[2],
    );
    // let _grid = Grid::new(Aabb::new(-half_widths, half_widths), params.shape);

    section("Reporting");
    info!("Known reactions:");
    for (name, val) in reactions.map().iter() {
        report!(val, name);
    }
    info!("Known species:");
    for (name, val) in species.map().iter() {
        report!(val, name);
    }
    info!("Known interfaces:");
    for (name, val) in interfaces.map().iter() {
        report!(val, name);
    }
    info!("Known materials:");
    for (name, val) in materials.map().iter() {
        report!(format!("{:?}", val), name);
    }
    info!("Known meshes:");
    for (name, _val) in meshes.map().iter() {
        info!("{}", name);
    }
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn prelude(params_path: &Path) -> Parameters {
    Parameters::load(&params_path)
}
