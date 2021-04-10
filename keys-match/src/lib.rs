#[macro_use]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use solana_program::pubkey::Pubkey;
use syn::spanned::Spanned;
use syn::{parse, Fields, Item, ItemStruct, TypePath};
const PATH: TypePath = TypePath::from(solana_program::pubkey::Pubkey);
#[proc_macro_attribute]
pub fn keys_match(meta: TokenStream, input: TokenStream) -> TokenStream {
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
fn count_fields(the_struct: &ItemStruct) -> u32 {
    match the_struct.fields {
        fields::Named(ref fields) => fields.named.iter().count(|field| field.ty == TYPE),
    }
}
