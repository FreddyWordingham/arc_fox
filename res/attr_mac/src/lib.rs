//! Attribute macros library.

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
    clippy::module_name_repetitions,
    clippy::option_expect_used,
    clippy::panic,
    clippy::result_expect_used,
    clippy::unreachable
)]
// Temporary suppression.
#![allow(clippy::missing_inline_in_public_items)]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream as TS1;
use proc_macro2::TokenStream as TS2;

/// Create the attribute macro Form.
#[proc_macro_attribute]
pub fn form(_metadata: TS1, input: TS1) -> TS1 {
    let input: TS2 = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, proc_mac::Form)]
        #input
    };
    output.into()
}
