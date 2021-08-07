use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();
    t.pass("tests/ui/simple-lens.rs");
    t.pass("tests/ui/lens-attributes.rs");
    t.compile_fail("tests/ui/with-empty-struct.rs");
    t.compile_fail("tests/ui/with-tuple-struct.rs");
    t.compile_fail("tests/ui/with-enum.rs");
    t.compile_fail("tests/ui/with-union.rs");

    t.compile_fail("tests/ui/with-snake_case.rs");
}
