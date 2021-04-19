#[test]
fn runner() {
    let t = trybuild::TestCases::new();
    // Failure case 1, the array is too short.
    t.compile_fail("tests/failures/01-fail-too-short.rs");
    // Fail to compile when the array is too long.
    t.compile_fail("tests/failures/02-fail-too-long.rs");
    t.compile_fail("tests/failures/03-fail-decode.rs");
    t.compile_fail("tests/failures/04-fail-field.rs")
}
