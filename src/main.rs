use solana_program::pubkey::Pubkey;

/// A sample struct with which to play.
#[derive(Debug)]
pub struct AddressBookEntry {
    name: String,
    pay_to: Pubkey,
}
impl AddressBookEntry {
    fn new(name: String, address: Pubkey) -> Self {
        Self {
            name: name,
            pay_to: address,
        }
    }
}
pub fn main() {
    let payto = AddressBookEntry::new("Richard".to_string(), Pubkey::new_unique());
    println!("{:#?}", payto);
}
