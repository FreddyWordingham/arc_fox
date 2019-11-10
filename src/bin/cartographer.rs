//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::{
    args,
    file::{Loadable, Saveable},
    geom::shapes::{Aabb, Mesh},
    index::Resolution,
    init::io_dirs,
    print, report,
    util::bin_name,
};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use rayon::prelude::*;

fn main() {
    title();
    args!(_bin_path: String);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!(in_dir.display(), "Input dir");

    print::section("Setup");

    print::section("Initialisation");
    let res = Resolution::new(101, 101, 101);
    let dom = Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0));
    let tris = Vec::load(&arc::dir::res::meshes().join("cube.obj"));
    let geom = Mesh::new(tris);

    print::section("Simulation");
    let intersection = intersect_test(1, &res, &dom, &geom);

    print::section("Post-Processing");

    print::section("Output");
    report!(out_dir.display(), "Output dir");
    intersection.save(&out_dir.join("intersection.nc"));

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}

use arc::geom::Collision;

#[pre(num_threads > 0)]
fn intersect_test(num_threads: usize, res: &Resolution, dom: &Aabb, shape: &Mesh) -> Array3<f64> {
    if num_threads == 1 {
        info!("Running as single thread.");

        let mut box_size = dom.widths();
        for (bw, n) in box_size.iter_mut().zip(res.arr()) {
            *bw /= *n as f64;
        }

        let mut intersections = Vec::with_capacity(res.total());
        for index in res {
            println!("{}", index);
            let mut mins = dom.mins().clone();
            mins.x += box_size.x * index.x() as f64;
            mins.y += box_size.y * index.y() as f64;
            mins.z += box_size.z * index.z() as f64;
            let maxs = mins + box_size;

            let bo = Aabb::new(mins, maxs);

            if shape.overlap(&bo) {
                intersections.push(1.0);
            } else {
                intersections.push(0.0);
            }
        }

        return Array3::from_shape_vec(*res.arr(), intersections).unwrap();
    }

    let thread_ids: Vec<usize> = (0..num_threads).collect();
    let mut cubes: Vec<Array3<f64>> = thread_ids.par_iter().map(|id| run_thread(*id)).collect();

    let mut cube = cubes.pop().unwrap();
    for c in cubes.iter() {
        cube += c;
    }

    cube
}

fn run_thread(_thread_index: usize) -> Array3<f64> {
    Array3::from_shape_vec((1, 2, 3), Vec::new()).unwrap()
}
