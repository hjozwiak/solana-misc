#[macro_use]
extern crate encodings_match;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// A sample struct with which to play.
pub struct AddressBookEntry {
    name: String,
    pay_to: Pubkey,
}

impl AddressBookEntry {
    fn new(name: String, address: Pubkey) -> Self {
        Self {
            name,
            pay_to: address,
        }
    }
}
pub fn main() {
    let payto = AddressBookEntry::new("Richard".to_string(), Pubkey::new_unique());
    let b58 = bs58::encode(payto.pay_to.to_bytes()).into_string();
    //ensure_match!(b58, payto.pay_to);
    // This should expand to something like this
    declare_local_pubkey!("my11111111111111111111111111111111111111111");
}
