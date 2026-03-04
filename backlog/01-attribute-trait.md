# Phase 1: `#[derive(Attribute)]`

## Goal

`#[derive(zyn::Attribute)]` generates a `parse(&syn::Attribute) -> syn::Result<Self>` method that extracts typed fields from an attribute's argument list. This is the primary API for parsing `#[my_attr(skip, rename = "foo", count = 3)]` into a typed struct.

## What This Replaces

The manual four-step extraction pattern using `AttrsExt` → `Args` → `Arg` → manual matching:

```rust
// Before: verbose, error-prone
let args = input.attrs.find_args("my_derive")?;
if let Some(arg) = args.get("rename") {
    if let Arg::Expr(_, expr) = arg {
        if let syn::Expr::Lit(lit) = expr {
            if let syn::Lit::Str(s) = &lit.lit {
                let rename = s.value();
            }
        }
    }
}
```

```rust
// After: declare the shape, derive the parsing
#[derive(zyn::Attribute)]
struct MyAttr {
    skip: bool,
    rename: Option<String>,
    #[zyn(default = 3)]
    count: i64,
}

let config: MyAttr = attr.parse_args()?;
```

## The `Attribute` Trait

Defined in `zyn-core`, one method:

```rust
pub trait Attribute: Sized {
    fn parse(attr: &syn::Attribute) -> syn::Result<Self>;
}
```

Blanket-implementable for any type that implements `syn::parse::Parse`:

```rust
impl<T: syn::parse::Parse> Attribute for T {
    fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
        attr.parse_args::<Self>()
    }
}
```

This means `#[derive(Attribute)]` generates `impl syn::parse::Parse` — the `Attribute` trait impl comes for free via the blanket.

## What `#[derive(Attribute)]` Generates

The derive generates two things:
1. `impl syn::parse::Parse for T` — the actual parsing logic
2. `impl Attribute for T` — comes free via blanket (or generated explicitly if blanket isn't used)

### For structs

```rust
#[derive(zyn::Attribute)]
struct SerdeConfig {
    rename_all: Option<String>,
    deny_unknown_fields: bool,
    default: bool,
}
```

Generates:

```rust
impl syn::parse::Parse for SerdeConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = input.parse::<zyn::Args>()?;

        Ok(Self {
            rename_all: /* Option<String> extraction from args */,
            deny_unknown_fields: args.has("deny_unknown_fields"),
            default: args.has("default"),
        })
    }
}

// Attribute trait — from blanket impl or generated directly:
// impl Attribute for SerdeConfig {
//     fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
//         attr.parse_args::<Self>()
//     }
// }
```

### Usage

```rust
use zyn::Attribute;

// Via the Attribute trait:
let config = SerdeConfig::parse(&attr)?;

// Or via syn's parse_args directly:
let config: SerdeConfig = attr.parse_args()?;

// Finding and parsing from an attribute slice:
if let Some(attr) = input.attrs.find_attr("serde") {
    let config = SerdeConfig::parse(attr)?;
}

// On each field:
for field in &fields {
    if let Some(attr) = field.attrs.find_attr("serde") {
        let field_config = FieldSerde::parse(attr)?;
    }
}

// On each variant:
for variant in &variants {
    if let Some(attr) = variant.attrs.find_attr("serde") {
        let variant_config = VariantSerde::parse(attr)?;
    }
}
```

### For enums

```rust
#[derive(zyn::Attribute)]
enum Casing {
    CamelCase,
    SnakeCase,
    PascalCase,
}
```

Generates `impl syn::parse::Parse` that matches the content as a flag word (variant names snake_cased for matching). The `Attribute` trait impl comes via the blanket.

## Type Mapping

How each field type is extracted from `Args`:

### Scalars

| Type | From `Flag(ident)` | From `Expr(_, lit)` | From `List(_, args)` | From `Lit(lit)` | From absent |
|---|---|---|---|---|---|
| `bool` | `true` | error | error | error | `false` |
| `String` | error | string `.value()` | error | string `.value()` | error |
| `i8`..`i128`, `u8`..`u128` | error | int literal | error | int literal | error |
| `f32`, `f64` | error | float literal | error | float literal | error |
| `char` | error | char literal | error | char literal | error |
| `syn::Ident` | the ident | error | error | error | error |
| `syn::Path` | as path | error | error | error | error |
| `syn::Expr` | error | the expr | error | error | error |
| `syn::LitStr` | error | string lit | error | string lit | error |
| `syn::LitInt` | error | int lit | error | int lit | error |

### Containers

| Type | From `Flag` | From `Expr` | From `List(_, args)` | From absent |
|---|---|---|---|---|
| `Option<T>` | `Some(T from word)` | `Some(T from arg)` | `Some(T from arg)` | `None` |
| `Vec<T>` | error | error | parse each inner arg | `vec![]` |
| `Args` | error | error | nested args directly | `Args::new()` |

### Nested structs

A field whose type also derives `Attribute` is parsed from a nested `List`:

```rust
#[derive(zyn::Attribute)]
struct Outer {
    inner: Inner,  // parsed from: my_attr(inner(a = 1, b = "x"))
}

#[derive(zyn::Attribute)]
struct Inner {
    a: i64,
    b: String,
}
```

## Field Annotations

| Annotation | Effect |
|---|---|
| `#[zyn(default)]` | Use `Default::default()` when absent |
| `#[zyn(default = value)]` | Use literal as default |
| `#[zyn(skip)]` | Don't extract; always default |
| `#[zyn(rename = "key")]` | Look for `key` instead of field name |

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/core/src/meta/attribute.rs` | **New** — type extraction helpers used by the generated `parse` method |
| `crates/core/src/meta/mod.rs` | Add `mod attribute; pub use attribute::*;` |
| `crates/derive/src/attribute.rs` | **New** — `#[derive(Attribute)]` expansion for structs and enums |
| `crates/derive/src/lib.rs` | Register `Attribute` derive macro |

## Tests

### Struct derive
- Full attribute parse with multiple typed fields
- Missing optional → `None`
- Missing required → error
- Unknown field → error
- `default`, `skip`, `rename` annotations
- Nested struct field

### Enum derive
- Flag word → variant
- Unknown variant → error

### Type extraction
- `bool` from flag / from absent
- `String` from string literal
- Integer types from int literal
- `Option<String>` present vs absent
- `Vec<String>` from nested list
- `syn::Ident` from flag ident

### Usage patterns
- `SerdeConfig::parse(&attr)` via the `Attribute` trait
- `attr.parse_args::<SerdeConfig>()` via syn's `Parse` directly
- Combined with `AttrsExt::find_attr` for lookup + parse
- Field-level and variant-level attribute parsing
