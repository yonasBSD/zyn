# Attribute Parsing

zyn provides a complete system for extracting typed data from proc macro attributes. The primary API is `#[derive(Attribute)]` — declare the shape of your attribute as a Rust struct and zyn generates the extraction logic.

## Overview

```rust
use zyn::FromInput;

#[derive(zyn::Attribute)]
#[zyn("builder", about = "Configure the builder derive")]
struct BuilderConfig {
    #[zyn(default = "build".to_string())]
    method: String,
    skip: bool,
    rename: Option<String>,
}

// Extraction in a proc macro:
let input: zyn::Input = derive_input.into();
let cfg = BuilderConfig::from_input(&input)?;
```

## Chapters

- [derive(Attribute)](./derive-attribute.md) — declare typed attribute structs
- [FromInput and Input](./from-input.md) — the extraction trait and input context type
- [Extractors](./extractors/README.md) — `Attr<T>`, `Fields<T>`, `Variants`, `Data<T>`, element input types
- [Arg and Args](./arg-args.md) — low-level attribute argument primitives
- [AttrExt and AttrsExt](./ext.md) — extension traits (legacy API)
