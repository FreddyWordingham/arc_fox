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

mod json;

use json::*;
use proc_macro::TokenStream;

/// Create the attribute macro form.
#[proc_macro_attribute]
pub fn form(metadata: TokenStream, input: TokenStream) -> TokenStream {
    form_impl(metadata, input)
}

/// Create the attribute macro json.
#[proc_macro_attribute]
pub fn json(metadata: TokenStream, input: TokenStream) -> TokenStream {
    json_impl(metadata, input)
}

/// Create the attribute macro save.
#[proc_macro_attribute]
pub fn save(metadata: TokenStream, input: TokenStream) -> TokenStream {
    save_impl(metadata, input)
}

/// Create the attribute macro load.
#[proc_macro_attribute]
pub fn load(metadata: TokenStream, input: TokenStream) -> TokenStream {
    load_impl(metadata, input)
}
