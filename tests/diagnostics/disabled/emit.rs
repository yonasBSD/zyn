#[zyn::element]
fn emit_error() -> zyn::TokenStream {
    bail!("broken input");
    zyn::TokenStream::new()
}

#[test]
fn fallback_emits_compile_error() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_error());
    zyn::assert_diagnostic_error!(output, "broken input");
    zyn::assert_tokens_contain!(output, "compile_error");
}

#[zyn::element]
fn emit_warning() -> zyn::TokenStream {
    warn!("deprecated usage");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_warning_prefixes_message() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_warning());
    zyn::assert_diagnostic_warning!(output, "deprecated usage");
    zyn::assert_tokens_contain!(output, "warning:");
}

#[zyn::element]
fn emit_note() -> zyn::TokenStream {
    note!("see documentation");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_note_prefixes_message() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_note());
    zyn::assert_diagnostic_note!(output, "see documentation");
    zyn::assert_tokens_contain!(output, "note:");
}

#[zyn::element]
fn emit_help() -> zyn::TokenStream {
    help!("try this instead");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_help_prefixes_message() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_help());
    zyn::assert_diagnostic_help!(output, "try this instead");
    zyn::assert_tokens_contain!(output, "help:");
}

#[zyn::element]
fn emit_multiple() -> zyn::TokenStream {
    error!("first");
    error!("second");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn fallback_multiple_errors() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@emit_multiple());
    zyn::assert_diagnostic_error!(output, "first");
    zyn::assert_diagnostic_error!(output, "second");
}
