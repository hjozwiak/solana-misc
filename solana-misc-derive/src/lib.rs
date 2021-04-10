extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use solana_program::pubkey::Pubkey;
use syn::parse::{Error, Parse, ParseStream, Result};

use syn::{parse_macro_input, Expr, ExprLit, ExprStruct, Ident, Lit, Token};
struct ArgumentParser {
    given_string: Expr,
    structure_to_compare: Ident,
}
impl Parse for ArgumentParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let compare_string = input.parse()?;
        input.parse::<Token![,]>()?;
        let the_struct: Ident = input.parse()?;
        Ok(ArgumentParser {
            given_string: compare_string,
            structure_to_compare: the_struct,
        })
    }
}
/// Given a struct that has a field of type Pubkey, find this member, evaluate the expression, decode it as base58, and ensure that it is equal to a given string.
#[proc_macro]
pub fn ensure_encodings_are_equal(input: TokenStream) -> TokenStream {
    let ArgumentParser {
        given_string,
        structure_to_compare,
    } = parse_macro_input!(input as ArgumentParser);
    let gen = quote! {
        #structure_to_compare
    };
    gen.into()
}
