//! Geometry mathematical sub-sub-module.

pub mod aabb;
pub mod collide;
pub mod mesh;
pub mod parallelogram;
pub mod rectangle;
pub mod smooth_triangle;
pub mod sphere;
pub mod triangle;

pub use self::{
    aabb::*, collide::*, mesh::*, parallelogram::*, rectangle::*, smooth_triangle::*, sphere::*,
    triangle::*,
};
