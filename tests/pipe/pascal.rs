#[test]
fn from_lower() {
    let name = zyn::format_ident!("foo");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "Foo");
}

#[test]
fn from_snake() {
    let name = zyn::format_ident!("foo_bar");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "FooBar");
}

#[test]
fn from_camel() {
    let name = zyn::format_ident!("fooBar");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "FooBar");
}

#[test]
fn from_pascal() {
    let name = zyn::format_ident!("FooBar");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "FooBar");
}

#[test]
fn from_screaming() {
    let name = zyn::format_ident!("FOO_BAR");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "FooBar");
}

#[test]
fn consecutive_uppercase() {
    let name = zyn::format_ident!("http_response");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "HttpResponse");
}

#[test]
fn all_uppercase_word() {
    let name = zyn::format_ident!("FOO");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "Foo");
}

#[test]
fn single_char() {
    let name = zyn::format_ident!("a");
    let result = zyn::zyn!({ { name | pascal } });
    assert_eq!(result.to_string(), "A");
}
