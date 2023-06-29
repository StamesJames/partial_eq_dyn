#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_errors/*.rs");
    t.pass("tests/ui/compile_passes/*.rs")
}
