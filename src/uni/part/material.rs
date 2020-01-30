//! Material implementation.

use crate::{access, phys::Optics};
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Material physical properties.
#[json]
pub struct Material {
    /// Optical properties.
    optics: Optics,
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Optional reaction rate multiplier.
    reaction_multiplier: Option<f64>,
}

impl Material {
    access!(optics, Optics);
    access!(visc, Option<f64>);
    access!(reaction_multiplier, Option<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(optics: Optics, visc: Option<f64>, reaction_multiplier: Option<f64>) -> Self {
        Self {
            visc,
            reaction_multiplier,
            optics,
        }
    }
}

impl Display for Material {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let wavelength = 600.0e-9;
        let env = self.optics.env(wavelength);
        write!(fmt, "({}nm): {}\t", wavelength * 1.0e9, env)?;

        if let Some(visc) = self.visc {
            write!(fmt, "Permeable {} Pa·s\t", visc)?;
        } else {
            write!(fmt, "Impermeable.\t")?;
        }

        if let Some(mult) = self.reaction_multiplier {
            write!(fmt, "Reactive ({})", mult)?;
        } else {
            write!(fmt, "Inert.")?;
        }

        Ok(())
    }
}
