use zyn_core::ast::Element;

#[test]
fn does_not_suppress_output() {
    let result = zyn::syn::parse_str::<Element>("@warn \"test warning\" struct Foo;")
        .unwrap()
        .to_token_stream()
        .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_note_child_does_not_suppress_output() {
    let result = zyn::syn::parse_str::<Element>(
        "@warn \"deprecated\" { @note \"see migration guide\" } struct Foo;",
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_help_child_does_not_suppress_output() {
    let result = zyn::syn::parse_str::<Element>(
        "@warn \"deprecated\" { @help \"use new_api() instead\" } struct Foo;",
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_note_and_help_does_not_suppress_output() {
    let result = zyn::syn::parse_str::<Element>(
        "@warn \"deprecated\" { @note \"removed in v3\" @help \"migrate to new_api()\" } struct Foo;",
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}
