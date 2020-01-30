//! Regular implementation.

use crate::{
    access,
    dom::{observe_mat, Cell, Name, Set},
    geom::{Aabb, Mesh, Ray},
    uni::{Interface, Material},
};
use nalgebra::{Point3, Unit};
use ndarray::Array3;
use std::fmt::{Display, Formatter, Result};

/// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
const HIT_ANGLE_THRESHOLD: f64 = 1.0e-1;

/// Grid sized partition scheme.
pub struct Regular {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell>,
}

impl Regular {
    access!(bound, Aabb);
    access!(cells, Array3<Cell>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        bound: Aabb,
        shape: [usize; 3],
        inters: &Set<Interface>,
        meshes: &Set<Mesh>,
    ) -> Self {
        println!("Building regular grid...");

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(shape.iter()) {
            *w /= *n as f64;
        }

        let gen_ray = |p: &Point3<f64>| -> Ray {
            for inter in inters.map().values() {
                let mesh = meshes.map().get(inter.surf()).expect("Invalid mesh name.");
                for tri in mesh.tris() {
                    let tc = tri.tri().centre();
                    if bound.contains(&tc) {
                        let dir = Unit::new_normalize(tc - p);
                        if dir.dot(tri.tri().plane_norm()).abs() >= HIT_ANGLE_THRESHOLD {
                            return Ray::new(*p, dir);
                        }
                    }
                }
            }

            panic!("Unable to determine suitable tracing ray.");
        };

        let total_cells = shape[0] * shape[1] * shape[2];
        let mut cells = Vec::with_capacity(total_cells);
        let pb = indicatif::ProgressBar::new(total_cells as u64);
        for xi in 0..*shape.get(0).expect("Missing resolution.") {
            let x = cell_size
                .get(0)
                .expect("Missing resolution.")
                .mul_add(xi as f64, bound.mins().x);
            for yi in 0..*shape.get(1).expect("Missing resolution.") {
                let y = cell_size
                    .get(1)
                    .expect("Missing resolution.")
                    .mul_add(yi as f64, bound.mins().y);
                for zi in 0..*shape.get(2).expect("Missing resolution.") {
                    pb.inc(1);

                    let z = cell_size
                        .get(2)
                        .expect("Missing resolution.")
                        .mul_add(zi as f64, bound.mins().z);

                    let mins = Point3::new(x, y, z);
                    let maxs = mins + cell_size;

                    let cell_bound = Aabb::new(mins, maxs);
                    let cell_centre = cell_bound.centre();

                    let mat = observe_mat(inters, meshes, &bound, &gen_ray(&cell_centre))
                        .expect("Unable to observe material.");

                    cells.push(Cell::new(cell_bound, mat));
                }
            }
        }

        Self {
            bound,
            cells: Array3::from_shape_vec(shape, cells)
                .expect("Failed to convert cell vector to an array3."),
        }
    }

    /// Create a map of material keys.
    #[inline]
    #[must_use]
    pub fn mat_names(&self) -> Array3<&Name> {
        self.cells.map(Cell::mat)
    }

    /// Create a map of material references.
    #[inline]
    #[must_use]
    pub fn mat_refs<'a>(&self, mats: &'a Set<Material>) -> Array3<&'a Material> {
        self.cells.map(|c| mats.map().get(c.mat()).unwrap())
    }
}

impl Display for Regular {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let shape = self.cells.shape();

        write!(
            fmt,
            "[{} x {} x {}] {}\tmins: ({}, {}, {})\tmaxs: ({}, {}, {})",
            shape.get(0).expect("Missing shape dimension."),
            shape.get(1).expect("Missing shape dimension."),
            shape.get(2).expect("Missing shape dimension."),
            self.cells.len(),
            self.bound.mins().x,
            self.bound.mins().y,
            self.bound.mins().z,
            self.bound.maxs().x,
            self.bound.maxs().y,
            self.bound.maxs().z,
        )
    }
}
