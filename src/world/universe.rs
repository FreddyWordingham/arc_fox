//! Universal information structure.

use super::load_mat_map;
use crate::dir::res::mats;
use self_ref::self_referencing;
use std::sync::Arc;

#[derive(Debug)]
pub struct Universe<'a> {
    a: f64,
    b: &'a f64,
}

impl<'a> Universe<'a> {
    pub fn new() -> Self {
        // let mat_map = load_mat_map(&mats(), &vec!["air".to_string(), "fog".to_string()]);

        // let ent_map = load_ent_map(vec![
        //     (
        //         "block_start".to_string(),
        //         Shape::new_plane(Point3::new(0.3, 0.0, 0.0), -Vector3::x_axis()),
        //         &mat_map["air"],
        //         &mat_map["fog"],
        //     ),
        //     (
        //         "block_end".to_string(),
        //         Shape::new_plane(Point3::new(0.5, 0.0, 0.0), -Vector3::x_axis()),
        //         &mat_map["fog"],
        //         &mat_map["air"],
        //     ),
        // ]);

        Arc::try_unwrap(self_referencing!(Universe, {
            a = 1.0;
            b = &a;
        }))
        .unwrap()
    }
}
