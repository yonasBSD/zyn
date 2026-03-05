# Extractors

Extractors are types implementing `FromInput` that pull structured data from a proc macro `Input`. When used as element params, they are auto-resolved — not passed at the call site.

## `Attr<T>` and `Extract<T>`

Generic wrappers for any `FromInput` type:

```rust
pub struct Attr<T: FromInput>(pub T);
pub struct Extract<T: FromInput>(pub T);
```

Both delegate to `T::from_input(input)`. Use `Attr<T>` to signal the value comes from a named attribute; use `Extract<T>` as a general-purpose wrapper.

```rust
#[zyn::element]
fn my_element(
    cfg: zyn::Attr<MyConfig>,              // from a named attribute
    ident: zyn::Extract<syn::Generics>,    // general extraction
) -> proc_macro2::TokenStream { ... }
```

Access the inner value via `.0`: `cfg.0.my_field`.

## `Fields<T>`

Extracts `syn::Fields` from a struct input. Generic over `T: FromFields`:

```rust
pub struct Fields<T: FromFields = syn::Fields>(pub T);
```

| `T` | Behaviour |
|---|---|
| `syn::Fields` (default) | Returns any field shape |
| `syn::FieldsNamed` | Errors if not named fields |
| `syn::FieldsUnnamed` | Errors if not unnamed fields |

```rust
#[zyn::element]
fn struct_element(
    fields: zyn::Fields,                          // any shape → fields.0: syn::Fields
    named: zyn::Fields<syn::FieldsNamed>,         // named only → named.0: syn::FieldsNamed
) -> proc_macro2::TokenStream {
    for field in &fields.0 { /* ... */ }
}
```

Can also be used outside elements:

```rust
let fields = zyn::Fields::from_input(&input)?;
for field in &fields.0 { /* ... */ }
```

## `Variants`

Extracts `Vec<syn::Variant>` from an enum input:

```rust
pub struct Variants(pub Vec<syn::Variant>);
```

```rust
#[zyn::element]
fn enum_element(
    variants: zyn::Variants,
) -> proc_macro2::TokenStream {
    for variant in &variants.0 { /* ... */ }
}
```

## `Data<T>`

Re-parses the full input token stream as any `T: syn::parse::Parse`:

```rust
pub struct Data<T: syn::parse::Parse>(pub T);
```

```rust
#[zyn::element]
fn fn_element(
    func: zyn::Data<syn::ItemFn>,
) -> proc_macro2::TokenStream {
    let name = &func.0.sig.ident;
    zyn::zyn! { /* use name */ }
}
```

## `FromFields` Trait

```rust
pub trait FromFields: Sized {
    fn from_fields(fields: syn::Fields) -> syn::Result<Self>;
}
```

Implemented for `syn::Fields`, `syn::FieldsNamed`, `syn::FieldsUnnamed`. Implement this trait to create custom field-shape extractors for use with `Fields<T>`.
