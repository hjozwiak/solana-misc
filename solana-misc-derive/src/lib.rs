#[macro_use]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use solana_program::pubkey::Pubkey;
use syn::{parse, parse_str, Data, DataStruct, DeriveInput, Fields, Type, TypePath};
#[proc_macro_derive(KeysMatch)]
pub fn keys_match(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Determine if there are any fields of type solana_program::pubkey in this struct
    // If there is more than one, returnan error since it is unclear what to do with them
    // For simplicity, make sure the struct is of named fields
    match ast.data {
        Data::Struct(ref the_struct) => {
            let name = ast.ident;

            let matches = count_fields(the_struct);
            let gen = quote! {
                impl KeysMatch for #name {
                    fn num_matches() -> usize {
                        #matches
                    }
                }
            };
            gen.into()
        }
        _ => panic!(),
    }
}
fn count_fields(the_struct: &DataStruct) -> usize {
    let path: TypePath = match parse_str("solana_program::pubkey::Pubkey") {
        Ok(p) => p,
        Err(e) => panic!(),
    };

    match the_struct.fields {
        Fields::Named(ref fields) => fields
            .named
            .iter()
            .filter(|field| field.ty == Type::Path(path.clone()))
            .count(),
        _ => 0,
    }
}
