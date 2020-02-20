//! Hit-type enumeration.

use contracts::pre;

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
    /// Construct a new scattering event instance.
    #[pre(dist > 0.0)]
    pub fn new_scattering(dist: f64) -> Self {
        Self::Scattering(dist)
    }

    /// Construct a new cell crossing instance.
    #[pre(dist > 0.0)]
    pub fn new_cell(dist: f64) -> Self {
        Self::Cell(dist)
    }

    /// Construct a new interface crossing instance.
    #[pre(dist > 0.0)]
    pub fn new_interface(dist: f64) -> Self {
        Self::Interface(dist)
    }

    /// Construct a new interface crossing instance, followed by a potential cell crossing.
    #[pre(dist > 0.0)]
    pub fn new_interface_cell(dist: f64) -> Self {
        Self::InterfaceCell(dist)
    }

    /// Construct a new instance.
    #[pre(scat_dist > 0.0)]
    #[pre(cell_dist > 0.0)]
    #[pre(inter_dist.is_none() || inter_dist.unwrap() > 0.0)]
    #[pre(bump_dist > 0.0)]
    pub fn new(scat_dist: f64, cell_dist: f64, inter_dist: Option<f64>, bump_dist: f64) -> Self {
        if cell_dist <= scat_dist {
            if let Some(inter_dist) = inter_dist {
                if cell_dist <= (inter_dist + (bump_dist * 1.0)) {
                    return Self::new_interface_cell(inter_dist);
                }

                return Self::new_interface(inter_dist);
            }

            return Self::new_cell(cell_dist);
        }

        if let Some(inter_dist) = inter_dist {
            if inter_dist <= scat_dist {
                return Self::new_interface(inter_dist);
            }
        }

        Self::new_scattering(scat_dist)
    }
}
