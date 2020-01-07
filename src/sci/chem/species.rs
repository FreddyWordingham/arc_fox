//! Substance species.

/// Reactive species structure.
pub struct Species {
    /// Unique name.
    pub name: String,
    /// Optional diffusive radius [m].
    pub rad: Option<f64>,
}

impl Species {
    /// Construct a new instance.
    #[inline]
    pub const fn new(name: String, rad: Option<f64>) -> Self {
        Self { name, rad }
    }
}
