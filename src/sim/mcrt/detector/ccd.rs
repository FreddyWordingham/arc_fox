//! CCD detector structure.

use crate::{
    access,
    sci::{
        math::{
            geom::shape::Rectangle,
            rt::{Ray, Trace},
        },
        phys::Photon,
    },
    sim::mcrt::Detect,
    util::list::alphabet::Greek::{Alpha, Beta, Gamma},
};
use nalgebra::Point3;
use ndarray::Array2;

/// CCD detector used to form images.
pub struct Ccd {
    /// Detection surface.
    surf: Rectangle,
    /// Image data.
    data: Array2<f64>,
}

impl Ccd {
    access!(surf, Rectangle);
    access!(data, Array2<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Rectangle, res: [usize; 2]) -> Self {
        Self {
            surf,
            data: Array2::zeros(res),
        }
    }

    /// Determine the resolution.
    #[inline]
    #[must_use]
    pub fn res(&self) -> [usize; 2] {
        let px = *self
            .data
            .shape()
            .get(Alpha as usize)
            .expect("Missing vertex.");
        let py = *self
            .data
            .shape()
            .get(Beta as usize)
            .expect("Missing vertex.");

        [px, py]
    }

    /// Determine the uv coordinates for a given position.
    #[inline]
    #[must_use]
    pub fn uv(&self, p: &Point3<f64>) -> [f64; 2] {
        let u_edge = self
            .surf
            .para()
            .verts()
            .get(Beta as usize)
            .expect("Missing vertex.")
            - self
                .surf
                .para()
                .verts()
                .get(Alpha as usize)
                .expect("Missing vertex.");
        let v_edge = self
            .surf
            .para()
            .verts()
            .get(Gamma as usize)
            .expect("Missing vertex.")
            - self
                .surf
                .para()
                .verts()
                .get(Alpha as usize)
                .expect("Missing vertex.");

        let h_vec = p - self
            .surf
            .para()
            .verts()
            .get(0)
            .expect("Missing vertex index.");
        let h = h_vec.magnitude();
        let theta = (u_edge.dot(&h_vec) / (u_edge.magnitude() * h)).acos();

        let u = (h * theta.cos()) / u_edge.magnitude();
        let v = (h * theta.sin()) / v_edge.magnitude();

        [u, v]
    }
}

impl Detect for Ccd {
    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        self.surf.dist(ray)
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    fn capture(&mut self, phot: &Photon) {
        let [u, v] = self.uv(phot.ray().pos());
        let [px, py] = self.res();

        *self
            .data
            .get_mut([
                (u * (px as f64)).floor() as usize,
                (v * (py as f64)).floor() as usize,
            ])
            .expect("Invalid uv index calculated.") += phot.weight();
    }
}
