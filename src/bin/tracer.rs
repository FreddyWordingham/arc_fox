//! Rainbow ray-tracing example binary.

use arc::{
    args,
    file::io::{Load, Save},
    report,
    sci::math::{geom::Parallelogram, rt::trace::Trace},
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
    res: (usize, usize),
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
    report!(params.res);

    section("Simulation");
    info!("Tracing...");
    let dist = simulation(params.res);
    info!("Tracing complete.");

    section("Output");
    info!("Saving...");
    dist.save(&out_dir.join("dist.nc"));
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

fn simulation(res: (usize, usize)) -> Array2<f64> {
    let mut dists = Array2::zeros(res);

    let tar = Parallelogram::new([
        Point3::new(10.0, -1.0, -1.0),
        Point3::new(10.0, -1.0, 1.0),
        Point3::new(10.0, 1.0, -1.0),
    ]);

    let proj = Projector::new(Point3::origin(), Vector3::x_axis(), 90.0f64.to_radians());
    for xi in 0..res.0 {
        for yi in 0..res.1 {
            let index = (xi, yi);
            let ray = proj.cast(res, index);
            if let Some(dist) = tar.dist(&ray) {
                dists[index] = dist;
            } else {
                dists[index] = 0.0;
            }
        }
    }

    dists
}

use arc::sci::math::rt::Ray;
use nalgebra::{Unit, Vector3};

/// Projector caster used for forming views from ray casts.
pub struct Projector {
    /// Emission position.
    pos: Point3<f64>,
    /// Viewing direction.
    dir: Unit<Vector3<f64>>,
    /// Up direction.
    up: Unit<Vector3<f64>>,
    /// Right direction.
    right: Unit<Vector3<f64>>,
    /// Field of view across the horizontal direction.
    fov: f64,
}

impl Projector {
    /// Construct a new instance.
    /// The up direction is assumed to be the z-axis.
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>, fov: f64) -> Self {
        let up = Vector3::z_axis();

        Self::new_with_up(pos, dir, fov, up)
    }

    /// Construct a new instance with an explicit up direction.
    pub fn new_with_up(
        pos: Point3<f64>,
        dir: Unit<Vector3<f64>>,
        fov: f64,
        up: Unit<Vector3<f64>>,
    ) -> Self {
        let right = Unit::new_normalize(dir.cross(&up));

        Self {
            pos,
            dir,
            up,
            right,
            fov,
        }
    }

    /// Get a ray corresponding to a given pixel for a given resolution.
    pub fn cast(&self, res: (usize, usize), index: (usize, usize)) -> Ray {
        let aspect_ratio = res.0 as f64 / res.1 as f64;
        let fov_y = self.fov / aspect_ratio;

        let dx = self.fov * ((index.0 as f64 / (res.0 - 1) as f64) - 0.5);
        let dy = fov_y * ((index.1 as f64 / (res.1 - 1) as f64) - 0.5);

        let rot = nalgebra::Rotation3::from_axis_angle(&self.up, dx)
            * nalgebra::Rotation3::from_axis_angle(&self.right, dy);

        Ray::new(self.pos, rot * self.dir)
    }
}
