#![cfg(feature = "const-validation")]

#[test]
fn test_compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
