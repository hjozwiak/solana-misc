#[macro_use]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use solana_program::pubkey::Pubkey;
use syn::spanned::Spanned;
use syn::{parse, parse_str, Fields, Item, ItemStruct, Type, TypePath};
#[proc_macro_attribute]
pub fn keys_match(meta: TokenStream, input: TokenStream) -> TokenStream {
    let path: TypePath = match parse_str("solana_program::pubkey::Pubkey") {
        Ok(p) => p,
    };

    let the_item: Item = syn::parse(input).expect("Failed to parse.");
    match the_item {
        Item::Struct(ref the_struct) => {
            // Determine if there are any fields of type solana_program::pubkey in this struct
            // If there is more than one, returnan error since it is unclear what to do with them
            // For simplicity, make sure the struct is of named fields
        }
        _ => panic!(),
    }
}
fn count_fields(the_struct: &ItemStruct, path: TypePath) -> usize {
    match the_struct.fields {
        Fields::Named(ref fields) => fields
            .named
            .iter()
            .filter(|field| field.ty == Type::Path(path))
            .count(),
    }
}
