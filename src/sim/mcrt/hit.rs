//! Hit-type enumeration.

/// Hit enumeration implementation.
#[derive(Debug)]
pub enum Hit {
    /// Scattering event.
    Scattering(f64),
    /// Cell collision.
    Cell(f64),
    /// Interface collision.
    Interface(f64),
    /// Interface collision, followed by a close cell collision.
    InterfaceCell(f64),
}

impl Hit {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(scat_dist: f64, cell_dist: f64, inter_dist: Option<f64>, bump_dist: f64) -> Self {
        if cell_dist <= scat_dist {
            if let Some(inter_dist) = inter_dist {
                if cell_dist <= bump_dist.mul_add(2.0, inter_dist) {
                    return Self::InterfaceCell(inter_dist);
                }

                return Self::InterfaceCell(inter_dist);
            }

            return Self::Cell(cell_dist);
        }

        if let Some(inter_dist) = inter_dist {
            if inter_dist <= scat_dist {
                return Self::Interface(inter_dist);
            }
        }

        Self::Scattering(scat_dist)
    }
}
