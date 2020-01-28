//! Library core.

#![warn(
    clippy::all,
    // clippy::cargo
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
)]
#![allow(
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::module_name_repetitions,
    clippy::panic,
    clippy::result_expect_used,
    clippy::wildcard_enum_match_arm
)]

pub mod chem;
pub mod data;
pub mod dom;
pub mod file;
pub mod geom;
pub mod math;
pub mod phys;
pub mod sim;
pub mod util;
