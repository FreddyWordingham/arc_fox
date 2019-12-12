//! Main example function demonstrating core capabilities.

use arc::{
    args,
    file::io::Save,
    form,
    // report,
    // sci::{
    // math::{
    //     rt::Ray,
    //     shape::{Aabb, Aperture},
    // },
    // phys::Spectrum,
    // },
    sim::diffusion,
    // sim::evolve,
    // sim::mcrt,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    // world::{
    //     parts::{index_of_name, Light},
    //     Universe, UniverseBuilder,
    // },
};
use log::info;
use nalgebra::Vector3;
use ndarray::Array3;
use std::path::Path;

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

    let _form_path = Path::new(&form_path);
    let (_in_dir, out_dir) = io_dirs(None, None);

    let shape = [100, 100, 100];
    let dom = Vector3::new(1.0, 1.0, 1.0);
    let cell_size: Vec<f64> = dom.iter().zip(&shape).map(|(x, n)| x / *n as f64).collect();
    let cell_size = Vector3::from_vec(cell_size);
    let mut concs = Array3::from_elem(shape, 0.0);
    let coeffs = Array3::from_elem(shape, Some(1.0));
    for xi in 40..60 {
        for yi in 40..60 {
            for zi in 40..60 {
                concs[[xi, yi, zi]] = 1.0;
            }
        }
    }

    let time = 10.0;
    let steps = 100;
    let dt = time / steps as f64;
    for n in 0..steps {
        println!("{} / {}", n, steps);
        diffusion::step::step_species(4, &mut concs, &coeffs, &cell_size, dt);
        concs.save(&out_dir.join(format!("concs_{}.nc", n)));
    }

    // section("Loading");
    // report!("Input dir", in_dir.display());
    // report!(
    //     "Loading parameters from file",
    //     in_dir.join(form_path).display()
    // );
    // let form = Parameters::load(&in_dir.join(form_path));
    // let builder = UniverseBuilder::new(
    //     Aabb::new_centred(&Point3::origin(), &form.half_widths),
    //     form.res,
    //     &in_dir,
    //     &form.reactions,
    //     &form.interfaces,
    // );

    // section("Building");
    // let mut universe = Universe::build(form.num_threads, builder);

    // section("Setup");
    // arc::util::format::universe(&universe);

    // section("Simulation");
    // let light = Light::new(
    //     Box::new(Aperture::new(
    //         Ray::new(Point3::new(0.0, 0.0, 250.0e-6), -Vector3::z_axis()),
    //         15.0_f64.to_radians(),
    //     )),
    //     Spectrum::new_laser(630.0e-9),
    //     1.0,
    // );
    // let light_map = mcrt::run(form.num_threads, form.num_phot, &light, &universe);

    // let ppix_index = index_of_name(universe.species(), "ppix");
    // let cells = universe.grid_mut().cells_mut();
    // let recs = light_map.recs;
    // for (rec, cell) in recs.iter().zip(cells) {
    //     cell.state_mut().concs_mut()[ppix_index] = rec.dist_travelled;
    // }

    // for k in 0..=600 {
    //     println!("k: {}", k);
    //     let conc = universe.generate_conc_maps();
    //     conc.save(&out_dir.join(format!("{}_concs.nc", k)));
    //     diffusion::run(form.num_threads, 6.0, &mut universe);
    //     evolve::run(form.num_threads, 6.0, &mut universe);
    // }

    // section("Post-Processing");
    // let mat = universe.generate_mat_maps();
    // // let mcrt = light_map.generate_density_maps();

    // section("Output");
    // report!("Output dir", out_dir.display());
    // mat.save(&out_dir.join("materials.nc"));
    // // mcrt.save(&out_dir.join("mcrt.nc"));

    // section("Finished");
}
