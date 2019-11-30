//! Optical crossing structure.

use crate::sci::math::Normal;
use contracts::{post, pre};
use nalgebra::{Unit, Vector3};

/// Crossing structure implementation.
/// Optical interface Crossing information structure.
#[derive(Debug)]
pub struct Crossing {
    /// Probability of reflection.
    ref_prob: f64,
    /// Reflection direction.
    ref_dir: Unit<Vector3<f64>>,
    /// Transmission (refraction) direction.
    trans_dir: Option<Unit<Vector3<f64>>>,
}

impl Crossing {
    /// Construct a new instance.
    #[pre(inc.is_normal())]
    #[pre(face_norm.is_normal())]
    #[pre(n_curr >= 1.0)]
    #[pre(n_next >= 1.0)]
    pub fn new(
        inc: &Unit<Vector3<f64>>,
        face_norm: &Unit<Vector3<f64>>,
        n_curr: f64,
        n_next: f64,
    ) -> Self {
        let norm = if inc.dot(face_norm) > 0.0 {
            Unit::new_normalize(face_norm.as_ref() * -1.0)
        } else {
            face_norm.clone()
        };

        let ci = -inc.dot(&norm);
        let n = n_curr / n_next;

        let crit_ang = if n_curr <= n_next {
            None
        } else {
            Some((n_next / n_curr).asin())
        };

        let ref_prob;
        let trans_dir;
        if crit_ang.is_some() && (ci.acos() >= crit_ang.unwrap()) {
            ref_prob = 1.0;
            trans_dir = None;
        } else {
            let s2t = n.powi(2) * (1.0 - ci.powi(2));
            let ct = (1.0 - s2t).sqrt();

            ref_prob = Self::init_ref_prob(n_curr, n_next, ci, ct);
            trans_dir = Some(Self::init_trans_dir(inc, &norm, n, ci, ct));
        }

        Self {
            ref_prob,
            ref_dir: Self::init_ref_dir(inc, &norm, ci),
            trans_dir,
        }
    }

    /// Calculate the reflection probability.
    #[post(0.0 <= ret && ret <= 1.0)]
    fn init_ref_prob(n1: f64, n2: f64, ci: f64, ct: f64) -> f64 {
        let n1ci = n1 * ci;
        let n2ct = n2 * ct;
        let rn = ((n1ci - n2ct) / (n1ci + n2ct)).powi(2);

        let n2ci = n2 * ci;
        let n1ct = n1 * ct;
        let rt = ((n2ci - n1ct) / (n2ci + n1ct)).powi(2);

        (rn + rt) / 2.0
    }

    /// Calculate the reflection direction.
    #[post(inc.dot(&ret) < 0.0)]
    fn init_ref_dir(
        inc: &Unit<Vector3<f64>>,
        norm: &Unit<Vector3<f64>>,
        ci: f64,
    ) -> Unit<Vector3<f64>> {
        Unit::new_unchecked(inc.into_inner() + (2.0 * ci * norm.into_inner()))
    }

    /// Calculate the transmission direction.
    #[post(inc.dot(&ret) > 0.0)]
    fn init_trans_dir(
        inc: &Unit<Vector3<f64>>,
        norm: &Unit<Vector3<f64>>,
        n: f64,
        ci: f64,
        ct: f64,
    ) -> Unit<Vector3<f64>> {
        Unit::new_unchecked((n * inc.into_inner()) + ((n * ci) - ct) * norm.into_inner())
    }

    /// Get the reflection probability.
    #[post(0.0 <= ret && ret <= 1.0)]
    pub fn ref_prob(&self) -> f64 {
        self.ref_prob
    }

    /// Get the transmission probability.
    #[post(0.0 <= ret && ret <= 1.0)]
    pub fn trans_prob(&self) -> f64 {
        1.0 - self.ref_prob
    }

    /// Retrieve the reflection direction.
    pub fn ref_dir(&self) -> &Unit<Vector3<f64>> {
        &self.ref_dir
    }

    /// Retrieve the transmission direction.
    pub fn trans_dir(&self) -> &Option<Unit<Vector3<f64>>> {
        &self.trans_dir
    }
}
