#![feature(proc_macro_diagnostic)]

use proc_macro2::TokenStream;

#[proc_macro]
pub fn compile_mappings(input: TokenStream) -> TokenStream {}
