# Phase 4: Add #[zyn::attribute]

Add `#[zyn::attribute]` attribute macro that replaces `#[proc_macro_attribute]`.

## Usage

```rust
#[zyn::attribute]
fn my_attr(
    #[zyn(input)] item: zyn::syn::ItemFn,
    args: zyn::Args,
) -> zyn::TokenStream {
    // body
}
```

## Generated code

```rust
#[proc_macro_attribute]
pub fn my_attr(
    __zyn_args: proc_macro::TokenStream,
    __zyn_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: ::zyn::Input = ::zyn::Input::Item(
        ::zyn::parse_input!(__zyn_input as ::zyn::syn::Item)
    );
    let mut diagnostics = ::zyn::Diagnostics::new();
    // diagnostic macros, extractor bindings
    let args: zyn::Args = ::zyn::parse_input!(__zyn_args as zyn::Args);
    let __body = { /* user body */ };
    if diagnostics.has_errors() { return diagnostics.emit().into(); }
    __body.into()
}
```

Non-extractor params become args bindings (at most one).

## Changes

**New:** `crates/derive/src/attribute_macro.rs`

**Modified:** `crates/derive/src/lib.rs`
- Add `mod attribute_macro;`
- Add proc macro entry:
  ```rust
  #[proc_macro_attribute]
  pub fn attribute(
      args: proc_macro::TokenStream,
      input: proc_macro::TokenStream,
  ) -> proc_macro::TokenStream {
      attribute_macro::expand(args.into(), input.into()).into()
  }
  ```

## Verify
```
cargo test --workspace
```
