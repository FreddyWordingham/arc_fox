//! Peel-off based imaging.

use arc::{
    args,
    file::io::{Load, Save},
    form, report,
    sci::{math::shape::Aabb, phys::Spectrum},
    sim::{imager, imager::Camera},
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::{parts::Light, Universe, UniverseBuilder},
};
use log::info;
use nalgebra::{Point3, Unit, Vector3};
use std::path::Path;

form!(Parameters,
    num_threads: usize;
    num_phot: u64;
    half_widths: Vector3<f64>;
    res: [usize; 3];
    reactions: Vec<String>;
    interfaces: Vec<String>;
    power: u32
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
        Box::new(Point3::origin()),
        Spectrum::new_laser(630.0e-9),
        1.0,
    );
    // let light_map = mcrt::run(form.num_threads, form.num_phot, &light, &universe);
    let cam = {
        let d = 2.0;
        let pos = Point3::new(d, -d, d);
        let tar = Point3::origin();
        let dir = Unit::new_normalize(tar - pos);
        let max_ang = (dir.dot(&Unit::new_normalize(universe.grid().dom().maxs() - pos))).acos();
        Camera::new(pos, dir, max_ang)
    };
    let image = imager::run(
        form.num_threads,
        form.num_phot,
        &light,
        &universe,
        &cam,
        form.power,
    );

    // for k in 0..100 {
    //     diffusion::run(form.num_threads, 1.0, &mut universe);
    //     let conc = universe.generate_conc_maps();
    //     conc.save(&out_dir.join(format!("{}_concs.nc", k)));
    // }

    section("Post-Processing");
    let mat = universe.generate_mat_maps();
    // let mcrt = light_map.generate_density_maps();

    section("Output");
    report!("Output dir", out_dir.display());
    mat.save(&out_dir.join("materials.nc"));
    // mcrt.save(&out_dir.join("mcrt.nc"));
    image.save(&out_dir.join("image.nc"));

    section("Finished");
}
