#[test]
fn default_returns_same_tokens_as_zyn() {
    let a = zyn::zyn!(
        struct Foo;
    );
    let b = zyn::expand! { struct Foo; };
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn pretty_returns_same_tokens_as_zyn() {
    let a = zyn::zyn!(
        struct Foo;
    );
    let b = zyn::expand! { pretty => struct Foo; };
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn raw_returns_same_tokens_as_zyn() {
    let a = zyn::zyn!(
        struct Foo;
    );
    let b = zyn::expand! { raw => struct Foo; };
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn ast_returns_same_tokens_as_zyn() {
    let a = zyn::zyn!(
        struct Foo;
    );
    let b = zyn::expand! { ast => struct Foo; };
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn pretty_with_control_flow() {
    let flag = true;
    let a = zyn::zyn!(@if (flag) { struct Foo; });
    let b = zyn::expand! { pretty => @if (flag) { struct Foo; } };
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn pretty_with_for_loop() {
    let names = vec![zyn::format_ident!("a"), zyn::format_ident!("b")];
    let a = zyn::zyn!(@for (name in &names) { {{ name }}, });
    let b = zyn::expand! { pretty => @for (name in names) { {{ name }}, } };
    assert_eq!(a.to_string(), b.to_string());
}
