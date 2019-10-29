//! Geometric triangle structure with interpolated Phong normals.

use super::{Collidable, Cube, Ray, Traceable};
use crate::file::Loadable;
use nalgebra::{Isometry3, Point3, Unit, Vector3};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Parallel ray catch value.
const EPSILON: f64 = 1.0e-6;

/// Triangle in three-dimensions.
/// Commonly used to compose surfaces.
#[derive(Debug)]
pub struct Triangle {
    /// Normal vertex.
    normal: Unit<Vector3<f64>>,
    /// Vertex positions.
    verts: [Point3<f64>; 3],
    /// Vertex normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl Triangle {
    /// Construct a new instance.
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        Self {
            normal: Unit::new_normalize((verts[1] - verts[0]).cross(&(verts[2] - verts[0]))),
            verts,
            norms,
        }
    }

    /// Reference the normal vector.
    pub fn normal(&self) -> &Unit<Vector3<f64>> {
        &self.normal
    }

    /// Reference the vertex positions.
    pub fn verts(&self) -> &[Point3<f64>; 3] {
        &self.verts
    }

    /// Reference the vertex normals.
    pub fn norms(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.norms
    }

    /// Calculate the area of the triangle.
    pub fn area(&self) -> f64 {
        ((self.verts[1] - self.verts[0]).cross(&(self.verts[2] - self.verts[0]))).magnitude() / 2.0
    }

    /// Apply a transformation to the triangle.
    pub fn transform(&mut self, trans: &Isometry3<f64>) {
        self.normal = trans * self.normal;

        for vert in self.verts.iter_mut() {
            *vert = trans * *vert;
        }

        for norm in self.norms.iter_mut() {
            *norm = trans * *norm;
        }
    }
}

impl Traceable for Triangle {
    fn intersect(&self, ray: &Ray) -> bool {
        let e01 = self.verts[1] - self.verts[0];
        let e02 = self.verts[2] - self.verts[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return false;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return false;
        }

        let dist = f * e02.dot(&q);

        if dist < EPSILON {
            return false;
        }

        true
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let e01 = self.verts[1] - self.verts[0];
        let e02 = self.verts[2] - self.verts[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e02.dot(&q);

        if dist < EPSILON {
            return None;
        }

        Some(dist)
    }

    fn distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        let e01 = self.verts()[1] - self.verts()[0];
        let e02 = self.verts()[2] - self.verts()[0];

        let h = ray.direction().cross(&e02);
        let a = e01.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.verts()[0];
        let u = f * s.dot(&h);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = s.cross(&e01);
        let v = f * ray.direction().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = f * e02.dot(&q);

        if dist < EPSILON {
            return None;
        }

        let w = 1.0 - u - v;

        Some((
            dist,
            Unit::new_normalize(
                (self.norms[1].into_inner() * u)
                    + (self.norms[2].into_inner() * v)
                    + (self.norms[0].into_inner() * w),
            ),
        ))
    }
}

impl Collidable for Triangle {
    fn collides(&self, cube: &Cube) -> bool {
        let c = cube.centre();
        let e = cube.half_widths();

        let v0 = self.verts[0] - c;
        let v1 = self.verts[1] - c;
        let v2 = self.verts[2] - c;

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

        if !axis_test(&self.normal) {
            return false;
        }

        true
    }
}

impl Loadable for Vec<Triangle> {
    fn load(path: &Path) -> Self {
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
