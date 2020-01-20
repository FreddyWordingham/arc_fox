//! Triangle geometry structure.

use crate::{
    access,
    file::io::Load,
    sci::math::{
        geom::{Aabb, Collide},
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

/// Triangle structure implementation.
/// Forms meshes.
pub struct Triangle {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    access!(verts, [Point3<f64>; 3]);
    access!(plane_norm, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Point3<f64>; 3]) -> Self {
        let plane_norm = Unit::new_normalize(
            (verts.get(Beta as usize).expect("Invalid vertex index.")
                - verts.get(Alpha as usize).expect("Invalid vertex index."))
            .cross(
                &(verts.get(Gamma as usize).expect("Invalid vertex index.")
                    - verts.get(Alpha as usize).expect("Invalid vertex index.")),
            ),
        );

        Self { verts, plane_norm }
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        ab + bc + ca
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        let s = (ab + bc + ca) * 0.5;

        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Centre point.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        Point3::from(
            ((self
                .verts
                .get(Alpha as usize)
                .expect("Invalid vertex index")
                .to_homogeneous()
                + self
                    .verts
                    .get(Beta as usize)
                    .expect("Invalid vertex index")
                    .to_homogeneous()
                + self
                    .verts
                    .get(Gamma as usize)
                    .expect("Invalid vertex index")
                    .to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }

    /// Determine the intersection distance along a ray's direction.
    /// Also return the barycentric intersection coordinates.
    #[must_use]
    pub fn intersection_coors(&self, ray: &Ray) -> Option<(f64, [f64; 3])> {
        let verts = self.verts;

        let e1 = verts.get(Beta as usize).expect("Invalid vertex index.")
            - verts.get(Alpha as usize).expect("Invalid vertex index.");
        let e2 = verts.get(Gamma as usize).expect("Invalid vertex index.")
            - verts.get(Alpha as usize).expect("Invalid vertex index.");

        let d_cross_e2 = ray.dir().cross(&e2);
        let e1_dot_d_cross_e2 = e1.dot(&d_cross_e2);

        if e1_dot_d_cross_e2.abs() <= 0.0 {
            return None;
        }

        let inv_e1_dot_d_cross_e2 = 1.0 / e1_dot_d_cross_e2;
        let rel_pos = ray.pos() - verts.get(Alpha as usize).expect("Invalid vertex index.");
        let u = inv_e1_dot_d_cross_e2 * rel_pos.dot(&d_cross_e2);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = rel_pos.cross(&e1);
        let v = inv_e1_dot_d_cross_e2 * ray.dir().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = inv_e1_dot_d_cross_e2 * e2.dot(&q);

        if dist <= 0.0 {
            return None;
        }

        let w = 1.0 - (u + v);

        Some((dist, [u, v, w]))
    }
}

impl Collide for Triangle {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        let mut mins = self.verts[Alpha as usize];
        let mut maxs = mins;

        for v in self.verts.iter().skip(1) {
            for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                if *min > *v {
                    *min = *v;
                } else if *max < *v {
                    *max = *v;
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    #[must_use]
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

impl Trace for Triangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some((dist, self.plane_norm));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.plane_norm.dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.plane_norm.dot(ray.dir()) > 0.0, self.plane_norm))
        } else {
            None
        }
    }
}

impl Load for Vec<Triangle> {
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
            words.next();
            let fy = words.next().unwrap().parse::<usize>().unwrap() - 1;
            words.next();
            let fz = words.next().unwrap().parse::<usize>().unwrap() - 1;
            words.next();

            faces.push((fx, fy, fz));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(Triangle::new([verts[face.0], verts[face.1], verts[face.2]]));
        }

        tris
    }
}
