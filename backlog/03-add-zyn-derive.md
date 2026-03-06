# Phase 3: Add #[zyn::derive]

Add `#[zyn::derive]` attribute macro that replaces `#[proc_macro_derive]`.

## Usage

```rust
#[zyn::derive]
fn my_derive(
    #[zyn(input)] fields: zyn::Fields,
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
) -> zyn::TokenStream {
    zyn::zyn!(impl {{ ident }} { })
}
```

## Generated code

```rust
#[proc_macro_derive(MyDerive)]
pub fn my_derive(__zyn_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: ::zyn::Input = ::zyn::Input::from(
        ::zyn::parse_input!(__zyn_input as ::zyn::syn::DeriveInput)
    );
    let mut diagnostics = ::zyn::Diagnostics::new();
    // diagnostic macros (error!, warn!, note!, help!, bail!)
    // extractor bindings
    let __body = { /* user body */ };
    if diagnostics.has_errors() { return diagnostics.emit().into(); }
    __body.into()
}
```

## Args

- `#[zyn::derive]` — derive name = PascalCase of function name
- `#[zyn::derive("CustomName")]` — custom name
- `#[zyn::derive("CustomName", attributes(helper1, helper2))]` — with helper attributes

All params must be `#[zyn(input)]` extractors.

## Changes

**New:** `crates/derive/src/derive_macro.rs`

**Modified:** `crates/derive/src/lib.rs`
- Add `mod derive_macro;`
- Add proc macro entry:
  ```rust
  #[proc_macro_attribute]
  pub fn derive(
      args: proc_macro::TokenStream,
      input: proc_macro::TokenStream,
  ) -> proc_macro::TokenStream {
      derive_macro::expand(args.into(), input.into()).into()
  }
  ```

## Verify
```
cargo test --workspace
```
