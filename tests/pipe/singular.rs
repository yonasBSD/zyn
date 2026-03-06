#[test]
fn strips_trailing_s() {
    let name = zyn::format_ident!("users");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "user");
}

#[test]
fn preserves_double_s() {
    let name = zyn::format_ident!("class");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "class");
}

#[test]
fn no_trailing_s() {
    let name = zyn::format_ident!("item");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "item");
}

#[test]
fn ends_with_xes() {
    let name = zyn::format_ident!("foxes");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "fox");
}

#[test]
fn ends_with_ses() {
    let name = zyn::format_ident!("buses");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "bus");
}

#[test]
fn ends_with_ches() {
    let name = zyn::format_ident!("fetches");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "fetch");
}

#[test]
fn ends_with_shes() {
    let name = zyn::format_ident!("flashes");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "flash");
}

#[test]
fn ends_with_ies() {
    let name = zyn::format_ident!("categories");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "category");
}

#[test]
fn vowel_y_plural() {
    let name = zyn::format_ident!("monkeys");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "monkey");
}
