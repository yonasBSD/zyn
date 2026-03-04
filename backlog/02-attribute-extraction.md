# Phase 3: Attribute Extraction in `#[zyn::input]`

## Goal

`#[zyn::input]` non-forwarded fields are automatically extracted from a named attribute using the `Attribute` trait (Phase 2). Field-level `#[zyn(...)]` annotations control default values, skipping, and renaming.

## What This Replaces

The manual `AttrExt`/`AttrsExt`/`Args` workflow. After this phase, `AttrExt` and `AttrsExt` become internal implementation details. The `ext` feature flag may be deprecated.

## Solution

```rust
#[zyn::input(derive::struct, "my_derive")]
struct MyInput {
    // Forwarded from AST (Phase 1)
    ident: syn::Ident,
    fields: syn::Fields,

    // Extracted from #[my_derive(...)] via Attribute (Phase 2)
    skip: bool,
    #[zyn(rename = "name")]
    rename_to: Option<String>,
    #[zyn(default = "serde")]
    format: String,
}
```

The second argument to `#[zyn::input]` names the attribute to extract non-forwarded fields from.

### Derive: item-level vs field-level attributes

The struct above extracts from **item-level** attributes (`#[my_derive(...)]` on the struct/enum itself). For **field-level** attributes, define a separate struct using `#[derive(Attribute)]`:

```rust
#[derive(zyn::Attribute)]
struct FieldConfig {
    skip: bool,
    rename: Option<String>,
}

zyn! {
    @for (field in input.fields.iter()) {
        @if (!FieldConfig::from_attrs(&field.attrs, "my_derive")?.skip) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
```

### Attribute macros: args extraction

For attribute macros, the first stream IS the args. Non-forwarded fields are extracted from that stream directly:

```rust
#[zyn::input(attr)]
struct MyInput {
    item: syn::ItemFn,        // from second stream (forwarded)
    mode: String,             // from first stream: #[my_attr(mode = "fast")]
    verbose: bool,            // from first stream: #[my_attr(verbose)]
}
```

No attribute name needed for attr mode — the args stream IS the attribute content.

### Field Attributes

| Attribute | Effect |
|---|---|
| `#[zyn(default)]` | Use `Default::default()` when absent |
| `#[zyn(default = "value")]` | Use literal value as default |
| `#[zyn(skip)]` | Don't extract; always `Default::default()` |
| `#[zyn(rename = "key")]` | Look for `key` instead of field name |

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/input.rs` | Extend with `Attribute`-based extraction for non-forwarded fields |
| `crates/core/src/ext.rs` | Consider deprecating the `ext` feature flag |

## Tests

- **Derive struct**: item-level attribute extraction
- **Derive enum**: item-level + variant-level extraction
- **Attribute macro**: args extraction from first stream (no `#[zyn(attr)]` needed)
- Bool field from flag
- String field from key=value
- Option field present vs absent
- Default, skip, rename annotations
- Missing required field → compile error
- Nested struct field using `#[derive(Attribute)]`
