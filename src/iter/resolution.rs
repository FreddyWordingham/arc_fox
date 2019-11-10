//! Resolution iterator struct.

use crate::index::{Index, Resolution as IndexResolution};
use contracts::pre;

/// Index resolution iterator struct.
pub struct Resolution {
    /// Current index.
    n: usize,
    /// Resolution limit.
    res: IndexResolution,
}

impl Resolution {
    /// Construct a new instance.
    #[pre(res.x() > 0)]
    #[pre(res.y() > 0)]
    #[pre(res.z() > 0)]
    pub fn new(res: IndexResolution) -> Self {
        Self { n: 0, res }
    }
}

impl Iterator for Resolution {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.res.total() {
            return None;
        }

        let index = self.res.nth_index(self.n);
        self.n += 1;

        Some(index)
    }
}
