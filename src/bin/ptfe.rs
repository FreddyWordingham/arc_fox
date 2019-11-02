//! PTFE investigation.

use arc::{
    dir::init,
    file::Loadable,
    geom::{Aabb, Shape},
    phys::Material,
    report,
    util::{print, start_up::get_args},
    world::{Domain, EntMap, Entity, MatMap},
};
use contracts::pre;
use log::info;
use nalgebra::Point3;
use ndarray::Array3;
use std::{collections::HashMap, path::PathBuf};

fn main() {
    title();
    let (_args, _cwd, out) = start_up();

    print::section("Initialising");
    let mat_map = load_mat_map(vec!["intralipid", "ptfe"]);
    let ent_map = load_ent_map(
        vec![(
            "vial",
            vec![Box::new(Aabb::new(
                Point3::new(-0.5, -0.5, -0.5),
                Point3::new(0.5, 0.5, 0.5),
            ))],
            "ptfe",
            "intralipid",
        )],
        &mat_map,
    );
    let dom = Domain::new(
        [21, 21, 21],
        Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0)),
        &ent_map,
    );

    print::section("Simulation");
    let mut intersections = Vec::with_capacity(dom.total_cells());
    let mut is_ptfe = Vec::with_capacity(dom.total_cells());
    for cell in dom.cells() {
        if cell.is_empty() {
            intersections.push(0.0);
        } else {
            intersections.push(cell.ents().len() as f64);
        }

        if cell.mat().env(700.0e-9).inter_coeff > 100.0 {
            is_ptfe.push(1.0);
        } else {
            is_ptfe.push(0.0);
        }
    }
    let intersections = Array3::from_shape_vec(*dom.shape(), intersections).unwrap();
    let is_ptfe = Array3::from_shape_vec(*dom.shape(), is_ptfe).unwrap();

    print::section("Output");
    arc::file::saveable::save_as_netcdf(vec![("surfs", &intersections)], &out.join("surfs.nc"));
    arc::file::saveable::save_as_netcdf(vec![("ptfe", &is_ptfe)], &out.join("ptfe_map.nc"));
}

fn title() {
    print::title("PTFE");

    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let cwd = init::cwd("ptfe");
    report!(cwd.display(), "cwd");

    let out = init::output();
    report!(out.display(), "out");

    (args, cwd, out)
}

#[pre(!mats.is_empty())]
fn load_mat_map(mats: Vec<&'static str>) -> MatMap {
    let mat_dir = arc::dir::res::mats();

    let mut mat_map = HashMap::new();
    for name in mats {
        info!("Loading mat: {}", name);
        mat_map.insert(
            name,
            Material::load(&mat_dir.join(format!("{}.json", name))),
        );
    }

    mat_map
}

#[pre(!ents.is_empty())]
fn load_ent_map<'a>(
    ents: Vec<(
        &'static str,
        Vec<Box<dyn Shape>>,
        &'static str,
        &'static str,
    )>,
    mat_map: &'a MatMap<'a>,
) -> EntMap<'a> {
    let mut ent_map = HashMap::new();
    for (name, surfs, in_mat, out_mat) in ents {
        info!("Loading ent: {}", name);

        ent_map.insert(
            name,
            Entity::new(surfs, &mat_map[in_mat], &mat_map[out_mat]),
        );
    }

    ent_map
}
