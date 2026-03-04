use zyn::quote::quote;

#[zyn::pipe]
fn shout(input: String) -> zyn::proc_macro2::Ident {
    zyn::proc_macro2::Ident::new(
        &format!("{}_BANG", input.to_uppercase()),
        zyn::proc_macro2::Span::call_site(),
    )
}

#[test]
fn custom_pipe() {
    let name = zyn::quote::format_ident!("hello");
    let result = zyn::zyn!({ { name | shout } });
    let expected = quote!(HELLO_BANG);
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::pipe("yell")]
fn make_loud(input: String) -> zyn::proc_macro2::Ident {
    zyn::proc_macro2::Ident::new(
        &format!("{}__LOUD", input.to_uppercase()),
        zyn::proc_macro2::Span::call_site(),
    )
}

#[test]
fn custom_name_override() {
    let name = zyn::quote::format_ident!("hello");
    let result = zyn::zyn!({ { name | yell } });
    let expected = quote!(HELLO__LOUD);
    assert_eq!(result.to_string(), expected.to_string());
}
