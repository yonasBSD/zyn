# The `#[zyn(input)]` Attribute

Element parameters marked with `#[zyn(input)]` are automatically resolved from the `Input` context via `FromInput::from_input`. Parameters without this attribute are treated as props and must be passed at the call site.

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] item: zyn::DeriveStruct,   // extractor — resolved from input
    name: zyn::proc_macro2::Ident,            // prop — passed at @my_element(name = ...)
) -> zyn::proc_macro2::TokenStream {
    zyn::zyn! { /* ... */ }
}
```

Any type that implements `FromInput` can be used as an extractor parameter. This includes:

- All wrapper extractors: `Attr<T>`, `Extract<T>`, `Fields<T>`, `Variants`, `Data<T>`
- All input types: `ItemFn`, `ItemStruct`, `DeriveStruct`, `DeriveEnum`, etc.
- Built-in impls: `proc_macro2::Ident`, `syn::Generics`, `syn::Visibility`

Multiple extractors can be used in the same element:

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] attr: zyn::Attr<MyConfig>,
    #[zyn(input)] fields: zyn::Fields<syn::FieldsNamed>,
    label: zyn::proc_macro2::Ident,
) -> zyn::proc_macro2::TokenStream {
    zyn::zyn! { /* attr.my_field, fields.named, label all available */ }
}
```
