//! Smooth-triangle geometry structure.

use crate::{
    access,
    file::io::Load,
    sci::math::{
        geom::{Aabb, Collide, Triangle},
        rt::{Ray, Trace},
    },
    util::list::alphabet::Greek::{Alpha, Beta, Gamma},
};
use nalgebra::{Point3, Unit, Vector3};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result::Result,
};

/// Triangle geometry with Phong normal interpolation.
/// Used to form meshes.
pub struct SmoothTriangle {
    /// Base triangle.
    tri: Triangle,
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl SmoothTriangle {
    access!(tri, Triangle);
    access!(norms, [Unit<Vector3<f64>>; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tri: Triangle, norms: [Unit<Vector3<f64>>; 3]) -> Self {
        if !norms.iter().all(|&n| n.dot(tri.plane_norm()) > 0.0) {
            panic!("Normals are not in direction with the plane.");
        }

        Self { tri, norms }
    }
}

impl Collide for SmoothTriangle {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.tri.bounding_box()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        self.tri.overlap(aabb)
    }
}

impl Trace for SmoothTriangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.tri.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.tri.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, [u, v, w])) = self.tri.intersection_coors(ray) {
            return Some((
                dist,
                Unit::new_normalize(
                    (self.norms[Beta as usize].into_inner() * u)
                        + (self.norms[Gamma as usize].into_inner() * v)
                        + (self.norms[Alpha as usize].into_inner() * w),
                ),
            ));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.tri.plane_norm().dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = ray.dir().dot(self.tri.plane_norm()) > 0.0;
            Some((dist, inside, norm))
        } else {
            None
        }
    }
}

impl Load for Vec<SmoothTriangle> {
    fn load(path: &Path) -> Self {
        let vertex_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("v "))
            .collect();

        let mut verts = Vec::with_capacity(vertex_lines.len());
        for line in vertex_lines {
            let mut words = line.split_whitespace();
            words.next();

            let px = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse px value.");
            let py = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse py value.");
            let pz = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse pz value.");

            verts.push(Point3::new(px, py, pz));
        }

        let normal_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("vn "))
            .collect();

        let mut norms = Vec::with_capacity(normal_lines.len());
        for line in normal_lines {
            let mut words = line.split_whitespace();
            words.next();

            let nx = words.next().unwrap().parse::<f64>().unwrap();
            let ny = words.next().unwrap().parse::<f64>().unwrap();
            let nz = words.next().unwrap().parse::<f64>().unwrap();

            norms.push(Unit::new_normalize(Vector3::new(nx, ny, nz)));
        }

        let face_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("f "))
            .collect();

        let mut faces = Vec::with_capacity(face_lines.len());
        for line in face_lines {
            let line = line.replace("//", " ");
            let mut words = line.split_whitespace();
            words.next();

            let fx = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let nx = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fy = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let ny = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let fz = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let nz = words.next().unwrap().parse::<usize>().unwrap() - 1;

            faces.push(((fx, fy, fz), (nx, ny, nz)));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(SmoothTriangle::new(
                Triangle::new([verts[(face.0).0], verts[(face.0).1], verts[(face.0).2]]),
                [norms[(face.1).0], norms[(face.1).1], norms[(face.1).2]],
            ));
        }

        tris
    }
}
