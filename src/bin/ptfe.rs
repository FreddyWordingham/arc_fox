//! Laura's ptfe experiment.

use arc::{
    args,
    file::io::{Load, Save},
    form, report,
    sci::{
        math::{
            rt::Ray,
            shape::{Aabb, Aperture},
        },
        phys::Spectrum,
    },
    sim::mcrt,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::{parts::Light, Universe, UniverseBuilder},
};
use log::info;
use nalgebra::{Point3, Vector3};
use std::path::Path;
//use ndarray-stats::Quartile

form!(Parameters,
    num_threads: usize;
    num_phot: u64;
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

    section("Simulation");
    let light = Light::new(
        Box::new(Aperture::new(
            Ray::new(Point3::new(-0.013, 0.0, 0.0), Vector3::x_axis()),
            0.01_f64.to_radians(),
        )),
        Spectrum::new_laser(830.0e-9),
        1.0,
    );
    let light_map = mcrt::run(form.num_threads, form.num_phot, &light, &universe);

    section("Post-Processing");
    let mat = universe.generate_mat_maps();
    let mcrt = light_map.generate_density_maps();
    // let total:mcrt::Record = light_map.recs.iter().sum();
    let mut total = mcrt::Record::default();
    for rec in light_map.recs.iter() {
        total += rec;
    }

    section("Output");
    report!("Output dir", out_dir.display());
    mat.save(&out_dir.join("materials.nc"));
    mcrt.save(&out_dir.join("mcrt.nc"));

    report!("Total detected Raman:", total.shifts);

    section("Finished");
}
