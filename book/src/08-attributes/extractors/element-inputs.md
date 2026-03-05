# Element Inputs

All zyn input wrapper types implement `FromInput` and can be used directly as `#[zyn(input)]` element parameters. This lets you inject the full typed input into your element.

## Derive Input Types

For derive macros (`#[derive(...)]`):

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] s: zyn::DeriveStruct,
) -> zyn::proc_macro2::TokenStream {
    let name = &s.ident;
    let field_count = s.data.fields.len();
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches | Fields |
|---|---|---|
| `DeriveInput` | Any derive input | `attrs`, `vis`, `ident`, `generics` |
| `DeriveStruct` | Struct only | `attrs`, `vis`, `ident`, `generics`, `data: syn::DataStruct` |
| `DeriveEnum` | Enum only | `attrs`, `vis`, `ident`, `generics`, `data: syn::DataEnum` |
| `DeriveUnion` | Union only | `attrs`, `vis`, `ident`, `generics`, `data: syn::DataUnion` |

## Item Input Types

For attribute macros and other item-level macros:

```rust
#[zyn::element]
fn fn_element(
    #[zyn(input)] item: zyn::ItemFn,
) -> zyn::proc_macro2::TokenStream {
    let name = &item.sig.ident;
    let args = &item.sig.inputs;
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches | Derefs to |
|---|---|---|
| `ItemInput` | Any item input | — (enum, use accessors) |
| `ItemFn` | `fn` | `syn::ItemFn` |
| `ItemStruct` | `struct` | `syn::ItemStruct` |
| `ItemEnum` | `enum` | `syn::ItemEnum` |
| `ItemUnion` | `union` | `syn::ItemUnion` |
| `ItemTrait` | `trait` | `syn::ItemTrait` |
| `ItemImpl` | `impl` | `syn::ItemImpl` |
| `ItemType` | `type` | `syn::ItemType` |
| `ItemMod` | `mod` | `syn::ItemMod` |
| `ItemConst` | `const` | `syn::ItemConst` |
| `ItemStatic` | `static` | `syn::ItemStatic` |
| `ItemUse` | `use` | `syn::ItemUse` |
| `ItemExternCrate` | `extern crate` | `syn::ItemExternCrate` |
| `ItemForeignMod` | `extern "C"` | `syn::ItemForeignMod` |
| `ImplItemFn` | fn inside `impl` | `syn::ImplItemFn` |
| `TraitItemFn` | fn inside `trait` | `syn::TraitItemFn` |

## Cross-Input Extraction

`ItemStruct`, `ItemEnum`, and `ItemUnion` also work with derive inputs — zyn reconstructs the `syn::Item*` type from the derive data:

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] s: zyn::ItemStruct,
) -> zyn::proc_macro2::TokenStream {
    // Works whether input is Input::Item or Input::Derive
    let name = &s.ident;
    zyn::zyn! { /* ... */ }
}
```

All other item types require an `Input::Item` context and will error on derive inputs.
