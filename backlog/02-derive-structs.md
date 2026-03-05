# Phase 2: `#[derive(Attribute)]` — Structs

## Goal

Generate `Attribute` implementations for structs. Supports attribute mode (struct has `#[zyn("name")]`) and argument mode (no struct-level `#[zyn(...)]`). Depends on Phase 1.

## Two Modes

The presence of `#[zyn("name")]` at the struct level determines the mode:

### Attribute mode

Struct has `#[zyn("name", ...)]`. Generates all three trait methods (`from_args`, `from_arg`, `attribute`) plus `about()`.

```rust
#[derive(Attribute)]
#[zyn("serde", unique, about = "Configure serialization")]
struct SerdeConfig {
    #[zyn(0, about = "the input path")]
    path: String,
    #[zyn("rename_all", about = "case transform for keys")]
    casing: Option<String>,
    #[zyn(about = "reject unknown fields")]
    deny_unknown_fields: bool,
    #[zyn(default = "json", about = "output format")]
    format: String,
}

let config = SerdeConfig::attribute(&input.attrs)?;
```

### Argument mode

Struct has no `#[zyn("name")]`. Generates only `from_args` and `from_arg`. Used as nested types within attribute structs.

```rust
#[derive(Attribute)]
struct Inner {
    a: i64,
    b: String,
}
```

## Struct-level Annotations

| Annotation | Effect |
|---|---|
| `#[zyn("name")]` | The attribute name to match (e.g. `"serde"` matches `#[serde(...)]`). Presence activates attribute mode. |
| `#[zyn(about = "...")]` | Description used in `about()` header and error messages |
| `#[zyn(unique)]` | Only one occurrence of this attribute allowed on an item. Multiple → error. Without this, multiple occurrences are merged. |

Combinable: `#[zyn("serde", unique, about = "Configure serialization")]`

## Field Annotations

| Annotation | Effect |
|---|---|
| `#[zyn(0)]` | Positional: consume `args[0]` (anonymous `Arg::Lit`). The integer is the positional index. |
| `#[zyn("key")]` | Name override: look for `args.get("key")` instead of the field name |
| (bare field) | Uses the field's own name: `args.get("field_name")` |
| `#[zyn(default)]` | Use `Default::default()` when absent |
| `#[zyn(default = value)]` | Use literal as default when absent |
| `#[zyn(skip)]` | Don't extract; always `Default::default()` |
| `#[zyn(about = "...")]` | Description used in error messages and generated `about()` |

Combinable: `#[zyn(0, default = ".", about = "working directory")]`

## Required vs Optional

- Non-`Option<T>` fields without `#[zyn(default)]` or `#[zyn(skip)]` → **required**
- `Option<T>` → always optional (absent → `None`)
- `bool` → always optional (absent → `false`)

## Duplicate Key Behavior

- Duplicate named keys within a single attribute → error
- `Vec<T>` fields collect multiple occurrences of the same key
- Multiple attribute occurrences on same item (e.g. two `#[serde(...)]`) → merged unless `#[zyn(unique)]`

## Generated Code Example

```rust
#[derive(Attribute)]
#[zyn("serde", unique, about = "Configure serialization")]
struct SerdeConfig {
    #[zyn(0, about = "the input path")]
    path: String,
    #[zyn("rename_all", about = "case transform")]
    casing: Option<String>,
    deny_unknown_fields: bool,
    #[zyn(default = "json", about = "output format")]
    format: String,
}
```

Generates:

```rust
impl ::zyn::Attribute for SerdeConfig {
    fn from_args(args: &::zyn::Args) -> ::syn::Result<Self> {
        Ok(Self {
            path: ::zyn::Attribute::from_arg(&args[0])?,
            casing: match args.get("rename_all") {
                Some(arg) => Some(::zyn::Attribute::from_arg(arg)?),
                None => None,
            },
            deny_unknown_fields: args.has("deny_unknown_fields"),
            format: match args.get("format") {
                Some(arg) => ::zyn::Attribute::from_arg(arg)?,
                None => ::std::string::String::from("json"),
            },
        })
    }

    fn from_arg(arg: &::zyn::Arg) -> ::syn::Result<Self> {
        match arg {
            ::zyn::Arg::List(_, args) => Self::from_args(args),
            _ => Err(::syn::Error::new(::proc_macro2::Span::call_site(), "expected list argument")),
        }
    }

    fn attribute(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
        let matches: Vec<_> = attrs.iter()
            .filter(|a| a.path().is_ident("serde"))
            .collect();

        if matches.len() > 1 {
            return Err(::syn::Error::new(::proc_macro2::Span::call_site(), "only one #[serde(...)] allowed"));
        }

        match matches.first() {
            Some(attr) => {
                let args: ::zyn::Args = attr.parse_args()?;
                Self::from_args(&args)
            }
            None => Self::from_args(&::zyn::Args::new()),
        }
    }
}

impl SerdeConfig {
    pub fn about() -> &'static str {
        "#[serde(...)]: Configure serialization\n\
         \n\
         Arguments:\n\
         [0] path: String (required) — the input path\n\
         rename_all: Option<String> — case transform\n\
         deny_unknown_fields: bool\n\
         format: String (default: \"json\") — output format"
    }
}
```

## Recursive Nesting

A field whose type also derives `Attribute` is parsed from a nested `List` arg via `T::from_arg(arg)`:

```rust
#[derive(Attribute)]
struct Inner { a: i64, b: String }

#[derive(Attribute)]
#[zyn("my_attr")]
struct Outer {
    inner: Inner,  // parsed from: my_attr(inner(a = 1, b = "x"))
}
```

`inner` matches `Arg::List("inner", args)` and calls `Inner::from_arg(arg)` → `Inner::from_args(args)`.

## `about()` Generation

Generated on attribute mode structs only:

- Header: `#[name(...)]: about text` (or just `#[name(...)]` if no struct-level `about`)
- Blank line, then `Arguments:` label
- One line per field (skip fields omitted):
  - Positional: `[N] name: Type (required|optional|default: "val") — about text`
  - Named: `name: Type (required|optional|default: "val") — about text`
  - `— about text` omitted if no field `about`

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/derive/src/attribute/mod.rs` | **New** — entrypoint `pub fn expand(input: TokenStream) -> TokenStream`, dispatches to `structs` or `enums` |
| `crates/derive/src/attribute/structs.rs` | **New** — struct codegen (both modes) |
| `crates/derive/src/lib.rs` | Register `#[proc_macro_derive(Attribute, attributes(zyn))]` |

## Tests

### Attribute mode struct
- Full extraction with multiple typed fields
- Positional args (`#[zyn(0)]`)
- Name override (`#[zyn("key")]`)
- Missing optional → `None`
- Missing required → error
- `default` annotation
- `skip` annotation → `Default::default()`
- `unique` → error on multiple attribute occurrences
- Non-unique → multiple attributes merged
- Absent attribute entirely → defaults apply

### Argument mode struct
- `from_args` extraction
- `from_arg` from `List` arg
- Nested within an attribute mode struct

### Recursive nesting
- Nested struct field parsed from `List` arg
- Multiple levels of nesting

### `about()` generation
- Attribute mode struct with `about` on struct and fields
- No `about` annotations → minimal output
- `skip` fields omitted
- Positional fields show `[N]` prefix
- Default values shown in parenthetical
