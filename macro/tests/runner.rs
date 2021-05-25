// Testing conditions for the procedural macro.
// There are four failure cases of which I am aware:
//1) The array is too short
// 2) The array is too long
// 3) There was some other failure in decoding
// 4) The first argument is not a struct field
// 5) The success case is when both keys are equal
//6) The partial success case is a successful compilation but the program panicked because they keys were not equal

#[test]
fn run_failures() {
    let t = trybuild::TestCases::new();
    // Failure case 1, the array is too short.
    t.compile_fail("tests/failures/01-fail-too-short.rs");
    // Fail to compile when the array is too long.
    t.compile_fail("tests/failures/02-fail-too-long.rs");
    // Fail to compile when the array can't be decoded
    t.compile_fail("tests/failures/03-fail-decode.rs");
    // Fail to compile when our first argument is not a field access expression
    t.compile_fail("tests/failures/04-fail-field.rs");
}

#[test]
fn run_success() {
    let t = trybuild::TestCases::new();
    t.pass("tests/successes/05-succeed.rs");
}

#[test]
#[should_panic]
fn run_partial_success() {
    let t = trybuild::TestCases::new();
    t.pass("successes/06-barf-when-unequal.rs");
}
