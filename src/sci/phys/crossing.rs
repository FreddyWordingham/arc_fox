//! Optical surface crossing information structure.

use nalgebra::{Unit, Vector3};

/// Photon surface crossing structure.
pub struct Crossing {
    /// Current refractive index.
    pub n1: f64,
    /// Next refractive index.
    pub n2: f64,
    /// Incoming ray direction.
    pub inc: Unit<Vector3<f64>>,
    /// Surface normal direction.
    pub norm: Unit<Vector3<f64>>,
}

impl Crossing {
    /// Construct a new instance.
    #[inline]
    pub fn new(n1: f64, n2: f64, inc: Unit<Vector3<f64>>, norm: Unit<Vector3<f64>>) -> Self {
        let cross = Self { n1, n2, inc, norm };

        if !cross.is_valid() {
            panic!("Failed to construct crossing instance.");
        }

        cross
    }

    /// Check the current configuration of the crossing is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.n1 >= 1.0 && self.n2 >= 1.0
    }

    /// Determine the normal direction corrected to face the correct way.
    fn norm(&self) -> Unit<Vector3<f64>> {
        if self.inc.dot(&self.norm) > 0.0 {
            Unit::new_normalize(self.norm.as_ref() * -1.0)
        } else {
            self.norm
        }
    }

    /// Calculate the critical angle.
    fn crit_ang(&self) -> Option<f64> {
        if self.n1 > self.n2 {
            Some((self.n2 / self.n1).asin())
        } else {
            None
        }
    }

    /// Calculate the cosine of the angle of incidence.
    fn cos_inc(&self) -> f64 {
        -self.inc.dot(&self.norm())
    }

    /// Calculate the square of the sine of the angle of transmission.
    fn sin_sq_trans(&self) -> f64 {
        (self.n1 / self.n2).powi(2) * (1.0 - self.cos_inc().powi(2))
    }

    /// Calculate the cosine of the angle of transmission.
    fn cos_trans(&self) -> f64 {
        (1.0 - self.sin_sq_trans()).sqrt()
    }

    /// Calculate the perpendicular reflection probability.
    fn ref_perp_prob(&self) -> f64 {
        let ci = self.cos_inc();
        let ct = self.cos_trans();

        ((self.n1 * ci - self.n2 * ct) / (self.n1 * ci + self.n2 * ct)).powi(2)
    }

    /// Calculate the perpendicular reflection probability.
    fn ref_para_prob(&self) -> f64 {
        let ci = self.cos_inc();
        let ct = self.cos_trans();

        ((self.n2 * ci - self.n1 * ct) / (self.n2 * ci + self.n1 * ct)).powi(2)
    }

    /// Calculate the probability of reflection.
    #[inline]
    pub fn ref_prob(&self) -> f64 {
        (self.ref_perp_prob() + self.ref_para_prob()) / 2.0
    }

    /// Calculate the probability of transmission.
    #[inline]
    pub fn trans_prob(&self) -> f64 {
        1.0 - self.ref_prob()
    }

    /// Calculate the reflection direction.
    #[inline]
    pub fn ref_dir(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.inc.into_inner() + (2.0 * self.cos_inc() * self.norm().as_ref()))
    }

    /// Calculate the transmission direction.
    #[inline]
    pub fn trans_dir(&self) -> Unit<Vector3<f64>> {
        let q = self.n1 / self.n2;
        Unit::new_normalize(
            (self.inc.into_inner() * q)
                + (self.norm().as_ref()
                    * ((q * self.cos_inc()) - (1.0 - self.sin_sq_trans()).sqrt())),
        )
    }
}
