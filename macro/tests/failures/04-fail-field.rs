#[macro_use]
extern crate assert_keys_equal_macro;
use solana_program::pubkey::Pubkey;
struct Tester {
    key: Pubkey,
}

fn main() {
    let test = Tester {
        key: Pubkey::new_unique(),
    };
    assert_pubkeys_equal!(
        "I should be a field.",
        "my1111111111111111111111111111111111111111111111111111"
    );
}
