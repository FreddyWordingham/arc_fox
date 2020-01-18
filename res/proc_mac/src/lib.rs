//! Procedural macros library.

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

mod hello_macro;
mod new;

use hello_macro::*;
use new::*;

extern crate proc_macro;

use crate::proc_macro::TokenStream;

/// Create the procedural macro `HelloMacro`.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Unable to parse token stream.");
    impl_hello_macro(&ast)
}

/// Create the procedural macro New.
#[proc_macro_derive(New)]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    impl_new_macro(input)
}
