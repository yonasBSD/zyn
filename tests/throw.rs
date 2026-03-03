#[test]
fn throw_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/throw.rs");
}
