//! Resolution iterator struct.

use crate::index::{Index, Resolution as IndexResolution};
use contracts::pre;

/// Index resolution iterator struct.
pub struct Resolution {
    /// Current index.
    i: usize,
    /// Resolution limit.
    res: IndexResolution,
}

impl Resolution {
    /// Construct a new instance.
    #[pre(res.x() > 0)]
    #[pre(res.y() > 0)]
    #[pre(res.z() > 0)]
    pub fn new(res: IndexResolution) -> Self {
        Self { i: 0, res }
    }
}

impl Iterator for Resolution {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.res.total() {
            return None;
        }

        let xi = self.i % self.res.x();
        let yi = ((self.i - xi) / self.res.x()) % self.res.y();
        let zi = (self.i - xi - (yi * self.res.x())) / (self.res.x() * self.res.y());

        self.i += 1;

        Some(Index::new(xi, yi, zi))
    }
}
