//! Laura's ptfe experiment.

use arc::{
    args,
    file::{Grid as FileGrid, Load, Save, Verse as FileVerse},
    report,
    geom::{
        Ray,
        Aabb, Aperture},
    phys::Spectrum,
    sim::mcrt,
    util::{
        io_dirs,
        exec,
        banner,
        //print::term::{section, title},
    },
    //world::{Light, Universe, UniverseBuilder},
};
use log::info;
use nalgebra::{Point3, Vector3};
use std::path::Path;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use attr::form;
//use ndarray-stats::Quartile

#[form]
// struct Parameters{
//     num_threads: usize,
//     num_phot: u64,
//     half_widths: Vector3<f64>,
//     res: [usize; 3],
//     reactions: Vec<String>,
//     interfaces: Vec<String>
// }
struct Parameters{
    num_threads: usize,
    num_phot: f64,
    verse: FileVerse,
    grid: FileGrid,
}

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Prelude");
    let params = prelude(&params_path);
    info!("loaded parameters file");

    banner::section("Loading");
    let verse = params.verse.form(&in_dir);

    banner::section("Building");
    let grid = params.grid.form(&verse);

    banner::section("Overview");
    info!("Universe contents:\n{}", verse);

    banner::section("Analysis");
    info!("Generating material maps...");
    let mat_maps = grid.mat_set(verse.mats());
    let total_cells = grid.cells().len();
    for (name, map) in mat_maps.map() {
        println!(
            "{:<32}\t{:<10}\t{}%",
            format!("{}:", name),
            map.sum(),
            map.sum() / total_cells as f64 * 100.0
        );
    }

    info!("Generating species maps...");
    let specs_refs = grid.specs_refs(verse.specs());
    for (name, map) in specs_refs.map() {
        println!("{:<32}\t{}", format!("{}:", name), map.map(|x| **x).sum());
    }

    info!("Plotting boundaries...");
    let boundaries = grid.boundaries();

    //banner::section("Output 1");
    //info!("Saving maps...");
    //for (name, map) in mat_maps.map() {
    //    map.save(&out_dir.join(format!("{}_map.nc", name)));
    //}
    //for (name, map) in specs_refs.map() {
    //    map.map(|x| **x)
    //        .save(&out_dir.join(format!("{}_map.nc", name)));
    //}
    //boundaries.save(&out_dir.join("boundaries.nc"));

    banner::section("Simulation");
    let light_map = arc::sim::mcrt::run(
        &arc::dom::Name::new("laser"),
        params.num_phot as u64,
        &verse,
        &grid,
    );

    banner::section("Output");
    light_map.save(&out_dir);
    //let mut tumour_dosage = 0.0;
    //let mat_names = grid.mat_names();
    //for (rec, name) in light_map.recs().iter().zip(mat_names.iter()) {
    //    if name.str() == "tumour" {
    //        tumour_dosage += rec.absorptions();
    //    }
    //}
    //report!(tumour_dosage);
    let total_shifts = light_map.recs()
        .map(|r| *r.shifts())
        .sum();
    let total_det_raman = light_map.recs()
        .map(|r| *r.det_raman())
        .sum();
    report!("Total created Raman", total_shifts);
    report!("Total detected Raman", total_det_raman);

    let mut file = BufWriter::new(OpenOptions::new().append(true).open(&out_dir.join("Ramans.txt")).unwrap());
    writeln!(file, "{}, {}", total_shifts, total_det_raman).unwrap();

    banner::section("Finished");
}
//     args!(
//         _bin_path: String;
//         form_path: String
//     );
//     let form_path = Path::new(&form_path);
//     let (in_dir, out_dir) = io_dirs(None, None);
//
//     banner::section("Loading");
//     //report!("Input dir", in_dir.display());
//     //report!(
//     //    "Loading parameters from file",
//     //    in_dir.join(form_path).display()
//     //);
//     let form = Parameters::load(&in_dir.join(form_path));
//     let builder = UniverseBuilder::new(
//         Aabb::new_centred(&Point3::origin(), &form.half_widths),
//         form.res,
//         &in_dir,
//         &form.reactions,
//         &form.interfaces,
//     );
//
//     banner::section("Building");
//     let universe = Universe::build(form.num_threads, builder);
//
//     banner::section("Setup");
//     arc::util::format::universe(&universe);
//
//     banner::section("Simulation");
//     let light = Light::new(
//         Box::new(Aperture::new(
//             Ray::new(Point3::new(-0.013, 0.0, 0.0), Vector3::x_axis()),
//             0.01_f64.to_radians(),
//         )),
//         Spectrum::new_laser(830.0e-9),
//         1.0,
//     );
//     let light_map = mcrt::run(form.num_threads, form.num_phot, &light, &universe);
//
//     banner::section("Post-Processing");
//     // //let mat = universe.generate_mat_maps();
//     // //let mcrt = light_map.generate_density_maps();
//     // // let total:mcrt::Record = light_map.recs.iter().sum();
//     // let mut total = mcrt::Record::default();
//     // for rec in light_map.recs.iter() {
//     //     total += rec;
//     // }
//     //
//     // section("Output");
//     // report!("Output dir", out_dir.display());
//     // mat.save(&out_dir.join("materials.nc"));
//     // mcrt.save(&out_dir.join("mcrt.nc"));
//     // report!("Total created Raman", total.shifts());
//     // report!("Total detected Raman", total.det_raman());
//     //
//     // let mut file = BufWriter::new(OpenOptions::new().append(true).open(&out_dir.join("Ramans.txt")).unwrap());
//     //writeln!(file, "{}, {}", total.shifts(), total.det_raman()).unwrap();
//
//
//     banner::section("Finished");
// }

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
