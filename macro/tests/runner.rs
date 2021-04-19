#[test]
fn runner() {
    let t = trybuild::TestCases::new();
    // Failure case 1, the array is too short.
    t.compile_fail("tests/failures/01-fail-too-short.rs");
    // Fail to compile when the array is too long.
    t.compile_fail("tests/failures/02-fail-too-long.rs");
    // Fail to compile when the array can't be decoded
    t.compile_fail("tests/failures/03-fail-decode.rs");
    // Fail to compile when our first argument is not a field access expression
    t.compile_fail("tests/failures/04-fail-field.rs");
    // Succeed when both keys are equal.
    t.pass("tests/successes/05-succeed.rs");
    #[should_panic]
    // Technically a success, but will panic at runtime since the keys are unequal
    t.pass("tests/successes/06-barf-when-unequal.rs");
}
