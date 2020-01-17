//! Library core.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![warn(
    clippy::all,
    // clippy::cargo
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
)]
#![allow(
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::option_expect_used,
    clippy::panic,
    clippy::result_expect_used,
    clippy::unreachable
)]
// Temporary suppression.
#![allow(clippy::missing_inline_in_public_items)]

pub mod data;
pub mod file;
pub mod ord;
pub mod sci;
pub mod util;
