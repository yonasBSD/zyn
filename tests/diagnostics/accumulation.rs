#[zyn::element]
fn three_levels() -> zyn::TokenStream {
    error!("first");
    warn!("second");
    note!("third");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn preserves_insertion_order() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@three_levels());
    zyn::assert_diagnostic_error!(output, "first");
    zyn::assert_diagnostic_warning!(output, "second");
    zyn::assert_diagnostic_note!(output, "third");
}

#[zyn::element]
fn all_four_levels() -> zyn::TokenStream {
    error!("err");
    warn!("warn");
    note!("note");
    help!("help");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn all_four_levels_accumulate() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@all_four_levels());
    zyn::assert_diagnostic_error!(output, "err");
    zyn::assert_diagnostic_warning!(output, "warn");
    zyn::assert_diagnostic_note!(output, "note");
    zyn::assert_diagnostic_help!(output, "help");
}

#[zyn::element]
fn error_and_warning() -> zyn::TokenStream {
    error!("from_a");
    warn!("from_b");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn merges_in_order() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@error_and_warning());
    zyn::assert_diagnostic_error!(output, "from_a");
    zyn::assert_diagnostic_warning!(output, "from_b");
}

#[zyn::element]
fn multiple_errors() -> zyn::TokenStream {
    error!("missing field `x`");
    error!("missing field `y`");
    error!("unknown argument `z`");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn accumulate_multiple_error_sources() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@multiple_errors());
    zyn::assert_diagnostic_error!(output, "missing field `x`");
    zyn::assert_diagnostic_error!(output, "missing field `y`");
    zyn::assert_diagnostic_error!(output, "unknown argument `z`");
}

#[zyn::element]
fn warn_only() -> zyn::TokenStream {
    warn!("just a warning");
    bail!();
    zyn::zyn!(
        struct Foo {}
    )
}

#[test]
fn bail_without_errors_does_not_stop() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@warn_only());
    zyn::assert_tokens_contain!(output, "Foo");
}

#[zyn::element]
fn error_then_bail() -> zyn::TokenStream {
    error!("fatal");
    bail!();
    zyn::zyn!(
        struct Foo {}
    )
}

#[test]
fn bail_with_errors_stops() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@error_then_bail());
    zyn::assert_diagnostic_error!(output, "fatal");
    assert!(output.tokens().is_empty());
}

#[zyn::element]
pub fn warn_with_output(name: zyn::syn::Ident) -> zyn::TokenStream {
    warn!("deprecated");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn warn_does_not_block_body() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @warn_with_output(name = zyn::format_ident!("my_fn"))
    );
    zyn::assert_tokens_contain!(output, "my_fn");
}

#[zyn::element]
pub fn note_and_help_with_output(name: zyn::syn::Ident) -> zyn::TokenStream {
    note!("processing `{}`", name);
    help!("consider adding #[derive(Debug)]");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn note_and_help_do_not_block_body() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @note_and_help_with_output(name = zyn::format_ident!("my_fn"))
    );
    zyn::assert_tokens_contain!(output, "my_fn");
}

#[zyn::element]
pub fn mixed_non_errors_with_output(name: zyn::syn::Ident) -> zyn::TokenStream {
    warn!("field will be removed");
    note!("see migration guide");
    help!("use `new_field` instead");
    zyn::zyn!(
        impl {{ name }} {
            fn validate(&self) -> bool { true }
        }
    )
}

#[test]
fn mixed_non_errors_do_not_block_body() {
    let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @mixed_non_errors_with_output(name = zyn::format_ident!("MyStruct"))
    );
    zyn::assert_tokens_contain!(output, "MyStruct");
    zyn::assert_tokens_contain!(output, "validate");
}
