use arc::{
    args,
    file::io::{Load, Save},
    sci::math::{
        rt::{Ray, Trace},
        shape::Mesh,
    },
    util::{info::exec, init::io_dirs, progress::SerialBar, term::title},
};
use log::info;
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use ndarray::Array2;

/// Horizontal resolution power.
const POWER: u32 = 11;

/// Field of view.
const FOV: f64 = 45.0;

fn main() {
    title(&exec::name());
    colog::init();

    args!(_bin_path: String;
    in_ang: f64);
    let (in_dir, out_dir) = io_dirs(None, None);

    let res = (2_usize.pow(POWER), 2 * 2_usize.pow(POWER));

    let fov_x = FOV.to_radians();
    let fov_y = fov_x * (res.1 as f64 / res.0 as f64);

    let dist = 1.5;
    let ang = in_ang.to_radians();
    let view_pos = Point3::new(dist * ang.sin(), dist * ang.cos(), 0.20);
    let view_dir = Unit::new_normalize(Point3::new(0.0, 0.0, 0.8) - view_pos);
    let sun_dir = Unit::new_normalize(Point3::new(10.0, -10.0, 10.0) - Point3::origin());

    let pitch_axis = Unit::new_normalize(view_dir.cross(&Vector3::z_axis()));
    let yaw_axis = Unit::new_normalize(pitch_axis.cross(&view_dir));

    let floor = Mesh::new(Vec::load(&in_dir.join("meshes/floor.obj")));
    let scene = Mesh::new(Vec::load(&in_dir.join("meshes/stars.obj")));
    let reindeer = Mesh::new(Vec::load(&in_dir.join("meshes/doe.obj")));
    let puddles = Mesh::new(Vec::load(&in_dir.join("meshes/puddles.obj")));
    let mut dists: Array2<f64> = Array2::zeros((res.1, res.0));
    let mut angle: Array2<f64> = Array2::zeros((res.1, res.0));
    let mut spec: Array2<f64> = Array2::zeros((res.1, res.0));
    let mut ang_x: Array2<f64> = Array2::zeros((res.1, res.0));
    let mut ang_y: Array2<f64> = Array2::zeros((res.1, res.0));
    let mut ang_z: Array2<f64> = Array2::zeros((res.1, res.0));

    let mut pb = SerialBar::new("Imaging", res.0 as u64 * res.1 as u64);
    for n in 0..res.0 {
        let x = (n as f64 / ((res.0 - 1) as f64)) - 0.5;
        let dx = fov_x * x;
        let yaw_rot = Rotation3::from_axis_angle(&yaw_axis, dx);

        for m in 0..res.1 {
            pb.inc();

            let y = (m as f64 / ((res.1 - 1) as f64)) - 0.5;
            let dy = fov_y * y;
            let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, dy);

            let dir = pitch_rot * yaw_rot * view_dir;
            let mut ray = Ray::new(view_pos, dir);

            loop {
                if let Some((dist, norm)) = reindeer.dist_norm(&ray) {
                    dists[[m, n]] = dist;
                    angle[[m, n]] = norm.dot(&dir).acos();
                    spec[[m, n]] = norm.dot(&sun_dir).acos();
                    ang_x[[m, n]] = norm.dot(&Vector3::x_axis()).acos();
                    ang_y[[m, n]] = norm.dot(&Vector3::y_axis()).acos();
                    ang_z[[m, n]] = norm.dot(&Vector3::z_axis()).acos();
                    break;
                } else if let Some(dist) = puddles.dist(&ray) {
                    let norm = Vector3::z_axis();
                    let ci = -ray.dir().dot(&norm);

                    ray.travel(dist);
                    ray.set_pos(ray.pos() + Vector3::new(0.0, 0.0, 0.001));

                    let ref_dir = Unit::new_unchecked(
                        ray.dir().into_inner() + (2.0 * ci * *Vector3::z_axis()),
                    );

                    ray.set_dir(ref_dir);
                } else if let Some((dist, norm)) = floor.dist_norm(&ray) {
                    dists[[m, n]] = dist;
                    angle[[m, n]] = norm.dot(&dir).acos();
                    spec[[m, n]] = norm.dot(&sun_dir).acos();
                    ang_x[[m, n]] = norm.dot(&Vector3::x_axis()).acos();
                    ang_y[[m, n]] = norm.dot(&Vector3::y_axis()).acos();
                    ang_z[[m, n]] = norm.dot(&Vector3::z_axis()).acos();
                    break;
                } else if let Some((dist, norm)) = scene.dist_norm(&ray) {
                    dists[[m, n]] = dist;
                    angle[[m, n]] = norm.dot(&dir).acos();
                    spec[[m, n]] = norm.dot(&sun_dir).acos();
                    ang_x[[m, n]] = norm.dot(&Vector3::x_axis()).acos();
                    ang_y[[m, n]] = norm.dot(&Vector3::y_axis()).acos();
                    ang_z[[m, n]] = norm.dot(&Vector3::z_axis()).acos();
                    break;
                } else {
                    break;
                }
            }
        }
    }

    dists.save(&out_dir.join(format!("dists.nc")));
    angle.save(&out_dir.join(format!("angles.nc")));
    spec.save(&out_dir.join(format!("spec.nc")));
    ang_x.save(&out_dir.join(format!("ang_x.nc")));
    ang_y.save(&out_dir.join(format!("ang_y.nc")));
    ang_z.save(&out_dir.join(format!("ang_z.nc")));
}
