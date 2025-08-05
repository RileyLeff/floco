#![cfg(all(feature = "const-validation", not(stable)))]

#[test]
fn test_compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
