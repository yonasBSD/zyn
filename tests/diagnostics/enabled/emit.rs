#[zyn::element]
fn emit_error() -> zyn::TokenStream {
    bail!("broken input");
    zyn::TokenStream::new()
}

#[test]
fn error_emits_tokens() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_error());
    assert!(!output.to_string().is_empty());
    zyn::assert_diagnostic_error!(output, "broken input");
}

#[zyn::element]
fn emit_warning() -> zyn::TokenStream {
    warn!("deprecated usage");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn warning_emits_tokens() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_warning());
    zyn::assert_diagnostic_warning!(output, "deprecated usage");
}

#[zyn::element]
fn emit_multiple() -> zyn::TokenStream {
    error!("first");
    error!("second");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn multiple_errors_all_emit() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_multiple());
    zyn::assert_diagnostic_error!(output, "first");
    zyn::assert_diagnostic_error!(output, "second");
}

#[zyn::element]
fn emit_nothing(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn no_diagnostics_passes_through() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @emit_nothing(name = zyn::format_ident!("my_fn"))
    );
    zyn::assert_tokens_contain!(output, "my_fn");
}
