//! Triangle structure.

use super::super::Transform;
use crate::{
    file::Load,
    list::alphabet::Greek::{Alpha, Beta, Gamma},
};
use contracts::pre;
use nalgebra::{Point3, Similarity3, Unit, Vector3};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Triangle structure implementation.
/// Forms meshes.
#[derive(Debug)]
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new instance.
    #[pre(norms.iter().all(|n| (n.magnitude() - 1.0).abs() < 1.0e-6))]
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Self::init_plane_norm(&verts);

        if !norms.iter().all(|&n| n.dot(&plane_norm) > 0.0) {
            panic!("Normals are not in direction with the plane.");
        }

        Self {
            verts,
            norms,
            plane_norm,
        }
    }

    /// Initialise the plane normal.
    fn init_plane_norm(verts: &[Point3<f64>; 3]) -> Unit<Vector3<f64>> {
        Unit::new_normalize(
            (verts[Beta as usize] - verts[Alpha as usize])
                .cross(&(verts[Gamma as usize] - verts[Alpha as usize])),
        )
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

impl Transform for Triangle {
    fn transform(&mut self, trans: &Similarity3<f64>) {
        for v in self.verts.iter_mut() {
            *v = trans.transform_point(v);
        }

        for n in self.norms.iter_mut() {
            *n = Unit::new_normalize(trans.transform_vector(n.as_ref()));
        }

        self.plane_norm = Self::init_plane_norm(&self.verts);
    }
}

impl Load for Vec<Triangle> {
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
