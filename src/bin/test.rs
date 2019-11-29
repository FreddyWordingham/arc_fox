//! Test function.

use arc::sci::math::{
    rt::{Emit, Trace},
    shape::Aabb,
};
use nalgebra::{Point3, Vector3};
use rand::thread_rng;

fn main() {
    let source = Point3::origin();
    let mut rng = thread_rng();
    let dom = Aabb::new_centred(&source, &Vector3::new(1.0, 1.0, 1.0));
    for _n in 0..1000 {
        let ray = source.emit(&mut rng);
        let hp = dom.hit_point(&ray).unwrap();

        println!(
            "{:<32}{:<32}{:<32}",
            format!("{},", hp.x),
            format!("{},", hp.y),
            hp.z
        );
    }
}
