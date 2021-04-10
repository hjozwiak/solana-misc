#[macro_use]
extern crate solana_misc_derive;
use solana_program::pubkey::Pubkey;
/// A sample struct with which to play.
trait KeysMatch {
    fn num_matches() -> usize;
}
#[derive(KeysMatch)]
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
    let _b58 = bs58::encode(payto.pay_to.to_bytes()).into_string();
}
