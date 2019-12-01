//! Species structure.

use crate::world::parts::Named;
use contracts::pre;

/// Reactive species structure implementation.
#[derive(Debug)]
pub struct Species {
    /// Name of the species.
    name: String,
    /// Optional radius of the molecule [m].
    rad: Option<f64>,
}

impl Species {
    /// Construct a new instance.
    #[pre(!name.is_empty())]
    #[pre(rad.is_none() || rad.unwrap() > 0.0)]
    pub fn new(name: String, rad: Option<f64>) -> Self {
        Self { name, rad }
    }

    /// Get the radius.
    pub const fn rad(&self) -> Option<f64> {
        self.rad
    }
}

impl Named for Species {
    /// Reference the name.
    fn name(&self) -> &str {
        &self.name
    }
}
