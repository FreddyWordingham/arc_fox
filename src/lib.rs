//! Library core.

#![doc(html_root_url = "https://freddywordingham.github.io/arc/")]
#![allow(dead_code)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::option_unwrap_used)]
#![allow(clippy::print_stdout)]
#![allow(clippy::result_unwrap_used)]
#![allow(clippy::similar_names)]
#![allow(clippy::zero_prefixed_literal)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]

pub mod file;
pub mod util;
