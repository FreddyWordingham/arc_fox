#![feature(test)]

extern crate test;

use arc::{dom::Aabb, geom::Shape, index::Layout3, proto::Entity as ProtoEntity, world::Universe};
use nalgebra::{Point3, Vector3};
use test::Bencher;

#[bench]
fn bench_universe_construction(b: &mut Bencher) {
    b.iter(|| {
        let uni = Universe::new(
            Layout3::new(17, 17, 17),
            Aabb::new_centred(&Point3::origin(), &Vector3::new(1.0, 1.0, 1.0)),
            vec![
                ProtoEntity::new(
                    Shape::new_plane(Point3::new(0.3, 0.0, 0.0), -Vector3::x_axis()),
                    "air",
                    "fog",
                ),
                ProtoEntity::new(
                    Shape::new_plane(Point3::new(0.5, 0.0, 0.0), -Vector3::x_axis()),
                    "fog",
                    "air",
                ),
            ],
        );
    });
}
