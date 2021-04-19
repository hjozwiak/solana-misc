#[macro_use]
extern crate solana_misc_macro;
use solana_program::pubkey::Pubkey;
struct Tester {
    key: Pubkey,
}

fn main() {
    let test = Tester {
        key: Pubkey::new_unique(),
    };
    assert_pubkeys_equal!(test.key, "ishouldfailtodecodethisstringstuff");
}
