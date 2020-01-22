//! Rainbow ray-tracing example binary.

use arc::{
    args,
    file::io::{Load, Save},
    report,
    sci::math::geom::Rectangle,
    sim::mcrt::detector::Ccd,
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
use ndarray::Array2;
use std::path::{Path, PathBuf};

#[form]
struct Parameters {
    num_phot: f64,
    res: usize,
    raindrop_rad: f64,
    ccd_dist: f64,
    ccd_size: f64,
    light_size: f64,
    light_power: f64,
    wavelength: f64,
}

fn main() {
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

    section("Manifest");
    let num_phot = params.num_phot as u64;
    report!(num_phot);

    section("Simulation");
    info!("Tracing...");
    let ccd = simulation(&params);
    info!("Tracing complete.");

    section("Output");
    info!("Saving...");
    ccd.save(&out_dir.join("ccd.nc"));
    info!("Saving complete.");

    println!("THis:\n{:?}", params);
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

fn simulation(params: &Parameters) -> Array2<f64> {
    let mut rng = thread_rng();

    let mut ccd = Ccd::new(
        Rectangle::from_points([
            Point3::new(params.ccd_dist, -params.ccd_size, -params.ccd_size),
            Point3::new(params.ccd_dist, params.ccd_size, -params.ccd_size),
            Point3::new(params.ccd_dist, -params.ccd_size, params.ccd_size),
        ]),
        [params.res, params.res],
    );

    let light = Light::new(
        Spectrum::Laser(params.wavelength),
        Box::new(Rectangle::from_points([
            Point3::new(
                -params.raindrop_rad * 2.0,
                -params.light_size,
                -params.light_size,
            ),
            Point3::new(
                -params.raindrop_rad * 2.0,
                params.light_size,
                -params.light_size,
            ),
            Point3::new(
                -params.raindrop_rad * 2.0,
                -params.light_size,
                params.light_size,
            ),
        ])),
    );

    let raindrop = Sphere::new(Point3::origin(), params.raindrop_rad);

    let num_phot = params.num_phot as u64;
    let mut pb = Bar::new("Photon loop", num_phot, 1);

    while let Some((start, end)) = pb.block(0, num_phot / 1000) {
        for _ in start..end {
            // pb.inc();
            let mut phot = light.emit(&mut rng, params.light_power / params.num_phot);

            loop {
                let rain_dist = raindrop.dist(phot.ray());
                let ccd_dist = ccd.dist(phot.ray());

                match Hit::new(rain_dist, ccd_dist) {
                    Hit::Nothing => {
                        break;
                    }
                    Hit::Interface(dist) => {
                        phot.travel(dist);
                        println!("Inter!");
                    }
                    Hit::Detection(dist) => {
                        phot.travel(dist);
                        ccd.capture(&phot);
                        println!("Capture!");
                    }
                }
            }
        }
    }

    ccd.data().clone()
}

/// Hit events to handle.
enum Hit {
    /// Escape.
    Nothing,
    /// Interface collision.
    Interface(f64),
    /// Detector collision.
    Detection(f64),
}

impl Hit {
    /// Construct a new instance.
    pub fn new(inter_dist: Option<f64>, detec_dist: Option<f64>) -> Self {
        if let Some(inter) = inter_dist {
            if let Some(detec) = detec_dist {
                if detec <= inter {
                    return Self::Detection(detec);
                }
            }

            return Self::Interface(inter);
        }

        if let Some(detec) = detec_dist {
            return Self::Detection(detec);
        }

        Self::Nothing
    }
}

use arc::sci::math::geom::Sphere;
use arc::sci::math::rt::{Ray, Trace};
use arc::sci::phys::Spectrum;
use arc::sim::mcrt::Detect;
use arc::sim::mcrt::Light;
use arc::util::pb::Bar;
use nalgebra::{Unit, Vector3};
use rand::thread_rng;
