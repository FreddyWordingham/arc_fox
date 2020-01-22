//! CCD detector structure.

use crate::sci::math::rt::Trace;
use crate::{
    access,
    sci::{
        math::{geom::Rectangle, rt::Ray},
        phys::Photon,
    },
    sim::mcrt::Detect,
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
        let px = self.data.shape()[0];
        let py = self.data.shape()[1];

        [px, py]
    }

    /// Determine the uv coordinates for a given position.
    #[inline]
    #[must_use]
    pub fn uv(&self, p: &Point3<f64>) -> [f64; 2] {
        let u_edge = self.surf.para().verts()[1] - self.surf.para().verts()[0];
        let v_edge = self.surf.para().verts()[2] - self.surf.para().verts()[0];

        let h_vec = p - self.surf.para().verts()[0];
        let h = h_vec.magnitude();
        let theta = ((u_edge.dot(&v_edge)) / (u_edge.magnitude() * h)).acos();

        let u = h * theta.cos();
        let v = h * theta.sin();

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
    #[must_use]
    fn detect(&mut self, phot: &Photon) {
        let [u, v] = self.uv(phot.ray().pos());
        let [px, py] = self.res();

        self.data[[(u * px as f64) as usize, (v * py as f64) as usize]] += phot.weight();
    }
}
