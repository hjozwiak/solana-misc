use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, ExprMacro, Ident};

extern crate proc_macro;
#[proc_macro]
pub fn ensure_match(input: TokenStream) -> TokenStream {}
