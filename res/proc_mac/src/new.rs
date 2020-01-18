//! Implementation function of the procedural macro `New`.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Add a simple constructor function which moves field variables (in order) into the structure.
pub fn impl_new_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let fields = input.fields.iter();
    let names = input.fields.iter().map(|f| &f.ident);

    let output = quote! {
        impl #name {
            pub fn new(#(#fields),*) -> Self {
                Self {
                    #(#names),*
                }
            }
        }
    };

    TokenStream::from(output)
}
