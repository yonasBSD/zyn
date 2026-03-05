# Phase 4: `#[element]` + `Attribute` Integration

## Goal

Allow `#[zyn::element]` function parameters to be automatically populated from the proc macro's input item instead of being passed as props. Depends on Phase 2.

## The Problem

Today, every parameter of an `#[element]` function becomes a struct field that must be filled by the caller as a prop: `@example(prop = val)`. There is no way to inject typed data derived from the proc macro's input item.

## Parameter Injection Annotations

Three `#[zyn(...)]` annotations control how a parameter is populated:

| Annotation | Meaning | Generated code |
|---|---|---|
| `#[zyn(input)]` | The whole input type — becomes a struct field, filled at call site | `let p = &self.p;` |
| `#[zyn(attr("name"))]` | Extract a specific attribute by ident via `Attribute::attribute()` | `let p = T::attribute(input.attrs()).unwrap_or_else(...)` |
| `#[zyn(input(field))]` | Access a field on the input type | `let p = &self.input.field;` |

Parameters with no annotation are **props** — struct fields filled by the caller as `@example(prop = val)`.

## Examples

```rust
#[derive(Attribute)]
#[zyn("my_attr")]
struct MyAttr {
    rename: Option<String>,
    skip: bool,
}

#[zyn::element]
fn example(
    #[zyn(input)] input: zyn::DeriveInput,
    #[zyn(attr("my_attr"))] attr: MyAttr,
    #[zyn(input(ident))] name: syn::Ident,
) -> zyn::proc_macro2::TokenStream {
    zyn::zyn! {
        // use input, attr, name here
    }
}
```

`input` and `name` become struct fields (call site provides `input`; `name` is derived from `input.ident`). `attr` is injected — not a prop.

## What `#[element]` generates

**Struct fields:** `#[zyn(input)]` params and unannotated prop params. Injected params (`attr`, `input(field)`) are not struct fields.

**`render()` body:**

```rust
impl ::zyn::Render for Example {
    fn render(&self) -> ::zyn::proc_macro2::TokenStream {
        let input = &self.input;
        let attr = <MyAttr as ::zyn::Attribute>::attribute(self.input.attrs())
            .unwrap_or_else(|e| e.to_compile_error());
        let name = &self.input.ident;
        // element body follows
    }
}
```

Error strategy for `attr(...)`: `.unwrap_or_else(|e| e.to_compile_error())` — no `Render` trait change needed.

## Input param requirement

An `#[element]` function with any `#[zyn(attr(...))]` or `#[zyn(input(...))]` param **must** also have exactly one `#[zyn(input)]` param whose type provides `.attrs()` / the accessed field. The known input types are:

- `zyn::DeriveInput`, `zyn::DeriveStruct`, `zyn::DeriveEnum`, `zyn::DeriveUnion`
- `zyn::ItemInput`, `zyn::ItemStruct`, `zyn::ItemEnum`, `zyn::ItemFn`, `zyn::ItemImpl`
- *(other `Item*` variants)*

If no `#[zyn(input)]` param is found but `attr(...)` or `input(...)` params exist, `#[element]` emits a compile error.

## Macro type inference (informational)

The type of the `#[zyn(input)]` param implies the kind of proc macro the element is designed for:
- `Derive*` types → derive macro context
- `Item*` types → attribute macro context
- No input param → pure template element

Not enforced at compile time in this phase — surfaced as a doc comment on the generated struct.

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/element.rs` | Parse the three `#[zyn(...)]` param annotations; classify params; generate struct fields and `render()` body accordingly |

## Tests

- `#[zyn(input)]` param → becomes a struct field
- `#[zyn(attr("name"))]` param → extracted from input attrs in render, not a prop
- `#[zyn(input(field))]` param → field access on input in render, not a prop
- `#[zyn(attr(...))]` with no `#[zyn(input)]` param → compile error
- Multiple `#[zyn(attr(...))]` params → all extracted from same input
- Unannotated params → unchanged prop behavior
- Attribute extraction failure at expand time → emits `compile_error!`
