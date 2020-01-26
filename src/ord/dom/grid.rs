//! Grid structure.

use crate::{
    access,
    ord::{dom::Cell, set::interfaces},
    sci::math::{geom::shape::Aabb, rt::Ray},
    uni::Verse,
};
use nalgebra::{Point3, Unit};
use ndarray::Array3;

/// Grid partition scheme.
pub struct Grid {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell>,
}

impl Grid {
    access!(bound, Aabb);
    access!(cells, Array3<Cell>);

    /// Construct a new instance.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn new(bound: Aabb, shape: [usize; 3], verse: &Verse) -> Self {
        let total_cells = shape[0] * shape[1] * shape[2];
        let mut cells = Vec::with_capacity(total_cells);

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(shape.iter()) {
            *w /= *n as f64;
        }

        let trace_point = {
            let mut tp: Option<Point3<f64>> = None;

            for interface in verse.interfaces().map().values() {
                let mesh = &verse.meshes()[interface.surf()];
                for tri in mesh.tris() {
                    let centre = tri.tri().centre();
                    if bound.contains(&centre) {
                        tp = Some(centre);
                        break;
                    }
                }
            }

            tp
        }
        .expect("Could not determine suitable trace target.");

        for xi in 0..*shape.get(0).expect("Invalid index.") {
            let x = cell_size
                .get(0)
                .expect("Invalid index.")
                .mul_add(xi as f64, bound.mins().x);
            for yi in 0..*shape.get(1).expect("Invalid index.") {
                let y = cell_size
                    .get(1)
                    .expect("Invalid index.")
                    .mul_add(yi as f64, bound.mins().y);
                for zi in 0..*shape.get(2).expect("Invalid index.") {
                    let z = cell_size
                        .get(2)
                        .expect("Invalid index.")
                        .mul_add(zi as f64, bound.mins().z);

                    let mins = Point3::new(x, y, z);
                    let maxs = mins + cell_size;

                    let cell_bound = Aabb::new(mins, maxs);
                    let cell_centre = cell_bound.centre();

                    let ray = Ray::new(cell_centre, Unit::new_normalize(trace_point - cell_centre));
                    let mat = interfaces::observe_material(
                        verse.interfaces(),
                        verse.meshes(),
                        &bound,
                        &ray,
                    )
                    .expect("Unable to determine central material.");

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
}
