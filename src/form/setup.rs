//! Setup form structure.

use crate::json;
use nalgebra::{Similarity3, Translation3, UnitQuaternion};
use serde::{Deserialize, Serialize};

/// Universe setup information.
#[derive(Serialize, Deserialize)]
pub struct Setup {
    /// Resolution of the grid.
    pub resolution: [usize; 3],
    /// Half widths of the universe.
    pub half_widths: [f64; 3],
    /// Entity information.
    pub ent_info: Vec<(String, String, Option<Similarity3<f64>>, String, String)>,
}

impl Setup {
    /// Construct an example instance.
    pub fn example() -> Self {
        let ent_info = vec![
            (
                "torus".to_string(),
                "torus".to_string(),
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, 0.0),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    0.75,
                )),
                "thick_fog".to_string(),
                "air".to_string(),
            ),
            (
                "upper-plane".to_string(),
                "plane".to_string(),
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, 1.5),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    2.5,
                )),
                "air".to_string(),
                "fog".to_string(),
            ),
            (
                "lower-plane".to_string(),
                "plane".to_string(),
                Some(Similarity3::from_parts(
                    Translation3::new(0.0, 0.0, -1.5),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                    2.5,
                )),
                "fog".to_string(),
                "air".to_string(),
            ),
        ];

        Self {
            resolution: [17, 17, 17],
            half_widths: [1.0, 1.0, 1.0],
            ent_info,
        }
    }
}

json!(Setup);
