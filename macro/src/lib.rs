//! Helper macros for determining if a pubkey is equal to a given Base58 encoded string.
///! Defines a proc macro to declare a pubkey locally, and a proc macro to determine whether or not a given string literal is equal to a given struct field.
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, ExprField, LitByte, LitStr, Token,
};

fn parse_pubkey_literal(
    input: ParseStream,
    pubkey_type: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let pubkey = if input.peek(syn::LitStr) {
        let pubkey_literal: LitStr = input.parse()?;
        parse_pubkey(&pubkey_literal, &pubkey_type)?
    } else {
        let expr: Expr = input.parse()?;
        quote! { #expr }
    };

    if !input.is_empty() {
        let stream: proc_macro2::TokenStream = input.parse()?;
        return Err(syn::Error::new_spanned(stream, "unexpected token"));
    }
    Ok(pubkey)
}

fn pubkey_to_tokens(
    id: &proc_macro2::TokenStream,
    pubkey_type: proc_macro2::TokenStream,
    tokens: &mut proc_macro2::TokenStream,
) {
    tokens.extend(quote! {
          #id
    });
}

struct KeyDecoder(proc_macro2::TokenStream);

impl Parse for KeyDecoder {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_pubkey_literal(input, quote! { ::solana_program::pubkey::Pubkey }).map(Self)
    }
}

impl ToTokens for KeyDecoder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        pubkey_to_tokens(&self.0, quote! { ::solana_program::pubkey::Pubkey }, tokens)
    }
}

#[proc_macro]
pub fn declare_local_pubkey(input: TokenStream) -> TokenStream {
    let pubkey = parse_macro_input!(input as KeyDecoder);
    TokenStream::from(quote! {#pubkey})
}

fn parse_pubkey(
    id_literal: &LitStr,
    pubkey_type: &proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let id_vec = bs58::decode(id_literal.value())
        .into_vec()
        .map_err(|_| syn::Error::new_spanned(&id_literal, "failed to decode base58 string"))?;
    let id_array = <[u8; 32]>::try_from(<&[u8]>::clone(&&id_vec[..])).map_err(|_| {
        syn::Error::new_spanned(
            &id_literal,
            format!("pubkey array is not 32 bytes long: len={}", id_vec.len()),
        )
    })?;
    let bytes = id_array.iter().map(|b| LitByte::new(*b, Span::call_site()));
    Ok(quote! {
        #pubkey_type::new_from_array(
            [#(#bytes,)*]
        )
    })
}

struct EqualityChecker {
    field: ExprField,
    pubkey: KeyDecoder,
}
impl Parse for EqualityChecker {
    fn parse(input: ParseStream) -> Result<Self> {
        let field: ExprField = input.parse()?;
        input.parse::<Token![,]>()?;
        let pubkey: KeyDecoder = input.parse()?;
        Ok(EqualityChecker { field, pubkey })
    }
}

#[proc_macro]
pub fn assert_encodings_match(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let EqualityChecker { field, pubkey } = parse_macro_input!(input as EqualityChecker);
    TokenStream::from(quote! {
        assert_eq!(#field, ##pubkey);
    })
}
