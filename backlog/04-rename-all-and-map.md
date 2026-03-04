# Phase 4: Rename All + Map

## Goal

Add `#[zyn(rename_all = "case")]` for bulk case transformation on attribute keys, and `#[zyn(map = pipe)]` for custom field-level transforms during parsing. Both integrate with zyn's existing case conversion and pipe systems.

## Problem

Attribute keys in Rust are snake_case but the source data or user preference may be camelCase, PascalCase, etc. Developers manually convert between conventions.

Also, extracted values sometimes need transformation before storage — lowercasing a string, parsing an ident from a string literal, etc.

## Solution

### `rename_all`

Applied at the struct level, transforms ALL attribute key lookups:

```rust
#[zyn::input("serde")]
#[zyn(rename_all = "camelCase")]
struct SerdeConfig {
    rename_all: Option<String>,    // looks for `renameAll` in the attribute
    deny_unknown: bool,            // looks for `denyUnknown`
}
```

Supported case values match zyn's built-in pipes: `snake`, `camel`, `pascal`, `screaming`, `kebab`.

### `map`

Applied at the field level, runs a zyn pipe or function on the extracted value:

```rust
#[zyn::input("my_attr")]
struct MyInput {
    #[zyn(map = snake)]
    name: String,               // extracted string is auto-snake_cased

    #[zyn(map = "custom_transform")]
    config: CustomConfig,       // calls custom_transform(raw_value)
}
```

When `map` references a built-in pipe name (`snake`, `pascal`, etc.), it uses zyn's case conversion directly. Otherwise it's treated as a function path.

## Design

### `rename_all` Implementation

In the generated `Parse` impl, each non-forwarded field's lookup key is transformed through the specified case conversion before querying `Args`:

```rust
// Without rename_all: args.get("deny_unknown")
// With rename_all = "camelCase": args.get("denyUnknown")
let key = zyn::case::to_camel("deny_unknown");
let deny_unknown = __zyn_args.has(&key);
```

### `map` Implementation

After extracting the raw value, the map function/pipe is applied:

```rust
let name = __zyn_args.get("name")
    .map(|a| zyn::case::to_snake(&extract_string(a)))
    .unwrap_or_default();
```

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/input.rs` | Parse `rename_all` from struct-level `#[zyn(...)]`; apply to key lookups. Parse `map` from field-level `#[zyn(...)]`; apply to extracted values. |

## Tests

- `rename_all = "camelCase"` transforms field lookups correctly
- `rename_all = "SCREAMING_SNAKE_CASE"` works
- `map = snake` applies case conversion to extracted string
- `map = "my_fn"` calls a custom function
- `rename_all` + per-field `rename` — field-level rename takes precedence
