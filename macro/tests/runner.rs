#[test]
fn runner() {
    let t = trybuild::TestCases::new();
    // Failure case 1, the array is too short.
    t.compile_fail("tests/failures/01-fail-too-short.rs");
}
