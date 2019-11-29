//! Test function.

use arc::sci::math::{
    rt::{Emit, Trace},
    shape::Aabb,
};
use nalgebra::{Point3, Vector3};
use rand::thread_rng;

fn main() {
    let source = Point3::new(3.0, -1.1, -1.3);
    let mut rng = thread_rng();
    let dom = Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0));

    let mut _hits = 0;
    let total = 10000;
    for _n in 0..total {
        let mut ray = source.emit(&mut rng);

        if let Some(dist) = dom.dist(&ray) {
            _hits += 1;

            let hops = 10;
            let delta = dist / hops as f64;
            for _ in 1..=hops {
                ray.travel(delta);

                println!(
                    "{:<32}{:<32}{:<32}",
                    format!("{},", ray.pos().x),
                    format!("{},", ray.pos().y),
                    ray.pos().z
                );
            }
        }
    }

    // println!("Fraction: {}", hits as f64 / total as f64);
}
