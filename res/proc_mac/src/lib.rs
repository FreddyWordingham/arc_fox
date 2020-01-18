extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            pub fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Noob)]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let fields = input.fields.iter();
    let names = input.fields.iter().map(|f| &f.ident);
    // let res = #(#fields),*;

    let output = quote! {
        impl #name {
            pub fn new(#(#fields),*) -> Self {
                Self {
                    #(#names),*
                }
            }
        }
    };

    //     // Return output tokenstream
    TokenStream::from(output)
}
