extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use solana_program::pubkey::Pubkey;
use syn::{parse_macro_input, ExprStruct, Ident};

/// Given a struct that has a field of type Pubkey, find this member, evaluate the expression, decode it as base58, and ensure that it is equal to a given string.
#[proc_macro]
pub fn ensure_encodings_are_equal(input: TokenStream) -> TokenStream {
    unimplemented!();
}
