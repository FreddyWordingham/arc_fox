//! Triangle structure.

use super::{Aabb, Collision, Transform, EPSILON};
use crate::{
    dim::Greek::{Alpha, Beta, Gamma},
    file::Loadable,
    rt::{Ray, Traceable},
};
use contracts::pre;
use log::info;
use nalgebra::{Point3, Similarity3, Unit, Vector3};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Triangle geometry with normal interpolation.
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new object.
    #[pre(norms.iter().all(|n| (n.magnitude() - 1.0).abs() < 1.0e-9))]
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Unit::new_normalize(
            (verts[Beta as usize] - verts[Alpha as usize])
                .cross(&(verts[Gamma as usize] - verts[Alpha as usize])),
        );

        if !norms.iter().all(|&n| n.dot(&plane_norm) > 0.0) {
            panic!("Normals are not in direction with the plane!");
        }

        Self {
            verts,
            norms,
            plane_norm,
        }
    }

    /// Reference the vertices.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the normal vectors.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }
}

impl Collision for Triangle {
    fn bounding_box(&self) -> Aabb {
        let mut mins = self.verts[Alpha as usize];
        let mut maxs = mins;

        for v in self.verts.iter() {
            for i in 0..3 {
                if mins[i] > v[i] {
                    mins[i] = v[i];
                } else if maxs[i] < v[i] {
                    maxs[i] = v[i];
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    fn overlap(&self, aabb: &Aabb) -> bool {
        let c = aabb.centre();
        let e = aabb.half_widths();

        let v0 = self.verts[Alpha as usize] - c;
        let v1 = self.verts[Beta as usize] - c;
        let v2 = self.verts[Gamma as usize] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vector3::x_axis();
        let u1 = Vector3::y_axis();
        let u2 = Vector3::z_axis();

        let axis_test = |axis: &Vector3<f64>| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);

            let r = (e.x * (u0.dot(axis)).abs())
                + (e.y * (u1.dot(axis)).abs())
                + (e.z * (u2.dot(axis)).abs());

            if (-(p0.max(p1).max(p2))).max(p0.min(p1).min(p2)) > r {
                return false;
            }

            true
        };

        if !axis_test(&u0) {
            return false;
        }
        if !axis_test(&u1) {
            return false;
        }
        if !axis_test(&u2) {
            return false;
        }

        let axis_u0_f0 = u0.cross(&f0);
        let axis_u0_f1 = u0.cross(&f1);
        let axis_u0_f2 = u0.cross(&f2);

        let axis_u1_f0 = u1.cross(&f0);
        let axis_u1_f1 = u1.cross(&f1);
        let axis_u1_f2 = u1.cross(&f2);

        let axis_u2_f0 = u2.cross(&f0);
        let axis_u2_f1 = u2.cross(&f1);
        let axis_u2_f2 = u2.cross(&f2);

        if !axis_test(&axis_u0_f0) {
            return false;
        }
        if !axis_test(&axis_u0_f1) {
            return false;
        }
        if !axis_test(&axis_u0_f2) {
            return false;
        }

        if !axis_test(&axis_u1_f0) {
            return false;
        }
        if !axis_test(&axis_u1_f1) {
            return false;
        }
        if !axis_test(&axis_u1_f2) {
            return false;
        }

        if !axis_test(&axis_u2_f0) {
            return false;
        }
        if !axis_test(&axis_u2_f1) {
            return false;
        }
        if !axis_test(&axis_u2_f2) {
            return false;
        }

        if !axis_test(&self.plane_norm) {
            return false;
        }

        true
    }
}

impl Transform for Triangle {
    fn trans(&mut self, trans: &Similarity3<f64>) {
        self.plane_norm = Unit::new_normalize(trans.transform_vector(self.plane_norm.as_ref()));

        for v in self.verts.iter_mut() {
            *v = trans.transform_point(v);
        }

        for n in self.norms.iter_mut() {
            *n = Unit::new_normalize(trans.transform_vector(n.as_ref()));
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: &Ray) -> bool {
        let e1 = self.verts[Beta as usize] - self.verts[Alpha as usize];
        let e2 = self.verts[2 as usize] - self.verts[Alpha as usize];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.pos - self.verts[Alpha as usize];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return false;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return false;
        }

        let dist = f * e2.dot(&q);

        if dist < EPSILON {
            return false;
        }

        true
    }

    fn dist(&self, ray: &Ray) -> Option<f64> {
        let e1 = self.verts[Beta as usize] - self.verts[Alpha as usize];
        let e2 = self.verts[Gamma as usize] - self.verts[Alpha as usize];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.pos - self.verts[Alpha as usize];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e2.dot(&q);

        if dist < EPSILON {
            return None;
        }

        Some(dist)
    }

    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let verts = self.verts;

        let e1 = verts[Beta as usize] - verts[Alpha as usize];
        let e2 = verts[Gamma as usize] - verts[Alpha as usize];

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.pos - verts[Alpha as usize];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e2.dot(&q);

        if dist < EPSILON {
            return None;
        }

        let w = 1.0 - u - v;

        Some((
            dist,
            Unit::new_normalize(
                (self.norms[Beta as usize].into_inner() * u)
                    + (self.norms[Gamma as usize].into_inner() * v)
                    + (self.norms[Alpha as usize].into_inner() * w),
            ),
        ))
    }
}

impl Loadable for Vec<Triangle> {
    fn load(path: &Path) -> Self {
        info!("Loading obj file: {}", path.display());

        let vertex_lines = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line.starts_with("v "));

        let mut verts = Vec::new(); // TODO Initialise to capacity.
        for line in vertex_lines {
            let mut words = line.split_whitespace();
            words.next();

            let px = words.next().unwrap().parse::<f64>().unwrap();
            let py = words.next().unwrap().parse::<f64>().unwrap();
            let pz = words.next().unwrap().parse::<f64>().unwrap();

            verts.push(Point3::new(px, py, pz));
        }

        let normal_lines = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line.starts_with("vn "));

        let mut norms = Vec::new(); // TODO Initialise to capacity.
        for line in normal_lines {
            let mut words = line.split_whitespace();
            words.next();

            let nx = words.next().unwrap().parse::<f64>().unwrap();
            let ny = words.next().unwrap().parse::<f64>().unwrap();
            let nz = words.next().unwrap().parse::<f64>().unwrap();

            norms.push(Unit::new_normalize(Vector3::new(nx, ny, nz)));
        }

        let face_lines = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line.starts_with("f "));

        let mut faces = Vec::new(); // TODO Initialise to capacity.
        for line in face_lines {
            let line = line.replace("//", " ");
            let mut words = line.split_whitespace();
            words.next();

            let fx = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fnx = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fy = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fny = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fz = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fnz = words.next().unwrap().parse::<usize>().unwrap() - 1;

            faces.push(((fx, fy, fz), (fnx, fny, fnz)));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(Triangle::new(
                [verts[(face.0).0], verts[(face.0).1], verts[(face.0).2]],
                [norms[(face.1).0], norms[(face.1).1], norms[(face.1).2]],
            ));
        }

        tris
    }
}
