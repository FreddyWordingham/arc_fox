//! PTFE investigation.

use arc::{
    // file::Loadable,
    file::Saveable,
    form::input::Ptfe as PtfeForm,
    geom::{Cube, Ray, Traceable},
    phy::Photon,
    util::{get_args, title},
};
use log::info;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{f64::consts::PI, path::Path};

fn main() {
    // Title.
    title("PTFE");
    colog::init();

    // Start up.
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let input_file_path = Path::new(&args[1]);

    // Load manifest.
    info!("Loading input form: {}", input_file_path.display());
    // let form = PtfeForm::example();
    let form = PtfeForm::load(input_file_path);

    let dir = form.dir().manifest();
    info!("Directory setup:\n{}", dir);

    let dom = form.dom().manifest();
    info!("Domain setup:\n{}", dom);

    let num_phot = form.num_phot();
    info!("Number of photons: {}", num_phot);
    let emission_pos = form.emission_pos();
    info!(
        "Emission position: [{}\t{}\t{}]",
        emission_pos.x, emission_pos.y, emission_pos.z
    );
    let emission_dir = form.emission_dir();
    info!(
        "Emission direction: [{}\t{}\t{}]",
        emission_dir.x, emission_dir.y, emission_dir.z
    );
    let emission_wavelength = form.emission_wavelength();
    info!("Emission wavelength: {}nm", emission_wavelength * 1.0e9);

    let intra_inter_coeff = form.intralipid_interaction_coeff();
    let intra_albedo = form.intralipid_albedo();
    let intra_raman_prob = form.intralipid_ramanisation_prob();
    let intra_asym = form.intralipid_asym();
    info!(
        "Intralipid:\nInteraction coeff : {}\nAlbedo:           : {}\nRaman chance:     : {}\nAsymmetry         : {}",
        intra_inter_coeff, intra_albedo, intra_raman_prob, intra_asym
    );

    let ptfe_inter_coeff = form.ptfe_interaction_coeff();
    let ptfe_albedo = form.ptfe_albedo();
    let ptfe_raman_prob = form.ptfe_ramanisation_prob();
    let ptfe_asym = form.ptfe_asym();
    info!(
        "PTFE:\nInteraction coeff : {}\nAlbedo:           : {}\nRaman chance:     : {}\nAsymmetry         : {}",
        ptfe_inter_coeff, ptfe_albedo, ptfe_raman_prob, ptfe_asym
    );

    let block = Cube::new(Point3::new(-0.1, -0.1, -0.1), Point3::new(0.1, 0.1, 0.1));

    let mut rng = thread_rng();

    // Simulation.
    simulate(
        num_phot,
        emission_pos,
        emission_dir,
        emission_wavelength,
        intra_inter_coeff,
        intra_albedo,
        intra_raman_prob,
        intra_asym,
        ptfe_inter_coeff,
        ptfe_albedo,
        ptfe_raman_prob,
        ptfe_asym,
        dom.boundary(),
        &block,
        &mut rng,
    );

    // Output.
    form.save(&dir.out().join("last_run.json"));
}

fn henyey_greenstein(rng: &mut ThreadRng, g: f64) -> f64 {
    let s = rng.gen_range(-1.0, 1.0);
    (1.0 / (2.0 * g)) * (1.0 + (g * g) - ((1.0 - (g * g)) / (1.0 + (g * s))).powi(2))
}

fn simulate(
    num_phot: u64,
    emission_pos: Point3<f64>,
    emission_dir: Unit<Vector3<f64>>,
    emission_wavelength: f64,
    intra_inter_coeff: f64,
    intra_albedo: f64,
    intra_raman_prob: f64,
    intra_asym: f64,
    ptfe_inter_coeff: f64,
    ptfe_albedo: f64,
    ptfe_raman_prob: f64,
    ptfe_asym: f64,
    dom: &Cube,
    block: &Cube,
    mut rng: &mut ThreadRng,
) {
    let mut total_raman = 0;

    let bar = arc::util::progress::bar(num_phot as u64);
    for p in 0..num_phot {
        bar.inc(1);

        let mut phot = Photon::new(Ray::new(emission_pos, emission_dir), emission_wavelength);

        let mut inter_coef = intra_inter_coeff;
        let mut _albedo = intra_albedo;
        let mut raman_prob = intra_raman_prob;
        let mut asym = intra_asym;
        let mut inside_ptfe = false;
        let mut ramanised = false;

        while dom.contained(phot.ray().origin()) {
            let domain_dist = dom.distance(phot.ray()).unwrap();
            let scat_dist = -rng.gen::<f64>().ln() / inter_coef;
            let block_dist = block.distance(phot.ray());

            if block_dist.is_some() {
                let block_dist = block_dist.unwrap();

                if (block_dist < domain_dist) && (block_dist < scat_dist) {
                    phot.travel(block_dist + 0.001); // Travel just beyond the boundary.

                    inside_ptfe = !inside_ptfe;

                    if inside_ptfe {
                        inter_coef = ptfe_inter_coeff;
                        _albedo = ptfe_albedo;
                        raman_prob = ptfe_raman_prob;
                        asym = ptfe_asym;
                    } else {
                        inter_coef = intra_inter_coeff;
                        _albedo = intra_albedo;
                        raman_prob = intra_raman_prob;
                        asym = intra_asym;
                    }
                } else if domain_dist < scat_dist {
                    phot.travel(domain_dist);
                    break;
                } else {
                    phot.travel(scat_dist);
                    phot.rotate(
                        henyey_greenstein(&mut rng, asym),
                        rng.gen_range(0.0, 2.0 * PI),
                    );

                    if !ramanised {
                        if rng.gen::<f64>() <= raman_prob {
                            ramanised = true;
                        }
                    }
                }
            } else {
                if domain_dist < scat_dist {
                    phot.travel(domain_dist);
                    break;
                } else {
                    phot.travel(scat_dist);
                    phot.rotate(
                        henyey_greenstein(&mut rng, asym),
                        rng.gen_range(0.0, 2.0 * PI),
                    );

                    if !ramanised {
                        if rng.gen::<f64>() <= raman_prob {
                            ramanised = true;
                        }
                    }
                }
            }

            if ramanised {
                total_raman += 1;
            }
        }
    }

    info!("Total raman photons: {}", total_raman);
}
