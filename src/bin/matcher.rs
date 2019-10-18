//! Matcher model setup.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use arc::{
    dir::materials,
    file::{Loadable, Saveable},
    form::{load, manifest::Matcher, Boundary},
    phy::Material,
    util::start_up,
    world::Boundary as wBoundary,
};
use log::{error, info};
use ndarray::Array3;
use std::{collections::HashMap, env::args, path::Path};

fn main() {
    // Start up.
    let (_cwd, out_dir) = start_up(&Path::new("cwd"), &Path::new("out"));

    // Command line arguments.
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        error!("Required call:\n{} <path/to/manifest.json>", &args[0]);
        return;
    }
    let input_file_path = Path::new(&args[1]);

    // Manifest file.
    let man = load::<Matcher>(input_file_path);
    // let man = Matcher::example();
    // man.save(Path::new("new.json"));

    let mat_map = load_mat_map(man.mat_list());

    let bound_map = load_bound_map(man.bound_list(), &mat_map);

    let grid = man.grid().manifest();

    let num_cells = grid.num_cells().clone();
    let total_cells = num_cells[0] * num_cells[1] * num_cells[2];

    let mut tri_map = Array3::from_elem(num_cells, Vec::new());
    let bar = arc::util::progress::bar(total_cells as u64);


    for xi in 0..num_cells[0] {
        for yi in 0..num_cells[1] {
            for zi in 0..num_cells[2] {
                bar.inc(1);

                let index = [xi, yi, zi];
                let cell_surf = grid.cell_surface(index);

                for boundary in bound_map.iter() {
                    let mut tri_list = Vec::new();
                    for tri in boundary.tris() {
                        if cell_surf.collides(tri) {
                            tri_list.push(tri);
                        }
                    }

                    if !tri_list.is_empty() {
                        tri_map[index].push((boundary.in_mat(), boundary.out_mat(), tri_list));
                    }
                }
            }
        }
    }

    let tri_num_map: Array3<f64> = tri_map.map(|tris| tris.len() as f64);
    tri_num_map.save(&out_dir.join("tri_num_map.nc"));
}

/// Load the given list of materials to the hashmap.
fn load_mat_map(mat_list: &Vec<String>) -> HashMap<String, Material> {
    let mat_dir = materials();
    let mut mat_map = HashMap::with_capacity(mat_list.len());

    for name in mat_list {
        info!("Loading {} material...", name);
        let path = mat_dir.join(format!("{}.json", name));

        mat_map.insert(name.clone(), Material::load(&path));
    }

    mat_map
}

/// Load the given list of boundaries into the hashmap.
fn load_bound_map<'a>(
    bound_list: &Vec<Boundary>,
    mat_map: &'a HashMap<String, Material>,
) -> Vec<wBoundary<'a>> {
    let mut bound_map = Vec::with_capacity(bound_list.len());

    for bound in bound_list {
        info!("Loading {} boundary...", bound.mesh());

        bound_map.push(bound.manifest(mat_map));
    }

    bound_map
}
