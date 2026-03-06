mod attribute;
mod common;
mod macros;

/// Expands a zyn template into a `TokenStream`.
#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::template::expand(input.into()).into()
}

/// Expands a zyn template with diagnostic output for debugging.
#[proc_macro]
pub fn debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::debug::expand(input.into()).into()
}

/// Defines a reusable template component that generates a struct implementing `Render`.
#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::element::expand(args.into(), input.into()).into()
}

/// Defines a custom pipe transform that generates a struct implementing `Pipe`.
#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::pipe::expand(args.into(), input.into()).into()
}

/// Defines a derive macro entry point that auto-parses `DeriveInput` into `Input`.
#[proc_macro_attribute]
pub fn derive(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::derive::expand(args.into(), input.into()).into()
}

/// Defines an attribute macro entry point that auto-parses the annotated item into `Input`.
#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::attribute::expand(args.into(), input.into()).into()
}

/// Derives the `Attribute` trait for typed attribute parsing from `#[attr(...)]` syntax.
#[proc_macro_derive(Attribute, attributes(zyn))]
pub fn derive_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(input.into()).into()
}
