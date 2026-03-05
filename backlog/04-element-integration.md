# Phase 4: `#[element]` + `FromInput` Extractors

## Goal

Element params typed as built-in extractor types (`Extract<T>`, `Attr<T>`, `Fields<T>`, `Variants`, `Data<T>`) are automatically populated from the `Input` context passed to `render` — no call-site props needed. All other params remain plain props. `Render::render` accepts `&Input` so elements always have access to the proc macro input. Depends on Phase 2.

## The Pattern

Built-in extractor types are detected by `#[element]` at macro expansion time. Each param is either:
- An extractor type (`Extract<T>`, `Attr<T>`, `Fields<T>`, `Variants`, `Data<T>`) → auto-resolved from `input` in `render`, not a struct field
- Anything else → plain prop, becomes a struct field filled at the call site

## Usage

```rust
#[derive(Attribute)]
#[zyn("my_attr")]
struct MyAttr {
    rename: Option<String>,
    skip: bool,
}

#[zyn::element]
fn example(
    cfg: zyn::Attr<MyAttr>,      // extractor — resolved from input, not a prop
    fields: zyn::Fields,         // extractor — resolved from input, not a prop
    label: String,               // prop — passed at @example(label = val)
) -> zyn::proc_macro2::TokenStream {
    // cfg.0, fields.0, label all available
    zyn::zyn! { ... }
}
```

## What `#[element]` generates

**Struct fields:** only prop params (e.g. `label: String`). Extractor params are not fields.

**`render(&self, input: &::zyn::Input)` body:** extractor params are resolved before prop bindings:

```rust
impl ::zyn::Render for Example {
    fn render(&self, input: &::zyn::Input) -> ::zyn::proc_macro2::TokenStream {
        let cfg = match <zyn::Attr<MyAttr> as ::zyn::FromInput>::from_input(input) {
            Ok(v) => v,
            Err(e) => { let __err: ::zyn::syn::Error = e.into(); return __err.to_compile_error(); }
        };
        let fields = match <zyn::Fields as ::zyn::FromInput>::from_input(input) { ... };
        let label = &self.label;
        // body follows
    }
}
```

## `Render` trait

```rust
pub trait Render {
    fn render(&self, input: &Input) -> proc_macro2::TokenStream;
}
```

`zyn!` always provides a default `input` in scope (an empty sentinel `Input::default()`). Users shadow it with their real proc macro input before calling `zyn!`:

```rust
let input: zyn::Input = real_derive_input.into();
zyn::zyn! { @example(label = "hello") }
```

## `Input` type

```rust
pub enum Input {
    Derive(DeriveInput),
    Item(ItemInput),
}
```

Accessors: `.attrs()`, `.ident()`, `.generics()`, `.vis()`
Implements: `Default`, `Parse`, `ToTokens`, `From<DeriveInput>`, `From<ItemInput>`

## Built-in Extractors

| Type | Extracts |
|---|---|
| `Extract<T: FromInput>` | Delegates to `T::from_input(input)` |
| `Attr<T: FromInput>` | Same as `Extract<T>` — signals attribute origin |
| `Fields<T: FromFields>` | Struct fields; `T` = `syn::Fields` (default), `syn::FieldsNamed`, `syn::FieldsUnnamed` |
| `Variants` | `Vec<syn::Variant>` from an enum input |
| `Data<T: Parse>` | Re-parses the full input as `T` |

Plus `FromInput` impls for: `proc_macro2::Ident`, `syn::Generics`, `syn::Visibility`

## Files Modified

| File | Change |
|---|---|
| `crates/core/src/input/mod.rs` | `Input` enum with `.attrs()`, `.ident()`, `.generics()`, `.vis()`, `Default`, `Parse`, `ToTokens` |
| `crates/core/src/extract.rs` | `FromInput`, `FromArg`, `FromFields` traits + all built-in impls + extractor wrappers |
| `crates/core/src/lib.rs` | `Render::render(&self, input: &Input)` |
| `crates/derive/src/element.rs` | Detect extractor params by type name; generate `FromInput::from_input(input)` bindings in `render` |
| `crates/core/src/ast/at/element_node.rs` | Pass `&input` to `render` at element call sites |
| `crates/derive/src/lib.rs` | `expand_template` defines `let input: Input = Default::default()` default |

## Tests

- `Attr<ElemAttr>` param → extracted from input, not a prop at call site
- Extraction failure → `compile_error!` at expand time
- Plain prop → unchanged behavior, still passed at call site
- `Render::render(struct, &real_input)` for testing with specific input
