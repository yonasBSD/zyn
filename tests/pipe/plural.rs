#[test]
fn basic() {
    let name = zyn::format_ident!("User");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "Users");
}

#[test]
fn already_ends_with_s() {
    let name = zyn::format_ident!("Status");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "Statuses");
}

#[test]
fn lowercase() {
    let name = zyn::format_ident!("item");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "items");
}

#[test]
fn ends_with_x() {
    let name = zyn::format_ident!("fox");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "foxes");
}

#[test]
fn ends_with_ch() {
    let name = zyn::format_ident!("fetch");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "fetches");
}

#[test]
fn ends_with_sh() {
    let name = zyn::format_ident!("flash");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "flashes");
}

#[test]
fn ends_with_consonant_y() {
    let name = zyn::format_ident!("category");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "categories");
}

#[test]
fn ends_with_vowel_y() {
    let name = zyn::format_ident!("monkey");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "monkeys");
}
