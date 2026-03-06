# Getting Started

## Installation

Add zyn to your proc-macro crate:

```toml
[dependencies]
zyn = "0.1.0"
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `derive` | yes | Template macros (`zyn!`, `debug!`), component macros (`#[zyn::element]`, `#[zyn::pipe]`), proc macro entry points (`#[zyn::derive]`, `#[zyn::attribute]`), `#[derive(Attribute)]`, extractors (`Attr<T>`, `Extract<T>`, `Fields<T>`, `Variants`, `Data<T>`), and diagnostics (`error!`, `warn!`, `note!`, `help!`, `bail!`) |
| `ext` | no | `AttrExt` and `AttrsExt` traits for `syn::Attribute` parsing |

## Basic Usage

Import the prelude and use the `zyn!` macro:

```rust
use zyn::prelude::*;

let name = &input.ident;
let output: zyn::TokenStream = zyn! {
    pub struct {{ name }} {
        id: u64,
    }
};
```

The `zyn!` macro returns a `zyn::TokenStream`. Everything outside `{{ }}` and `@` directives passes through as literal tokens, just like `quote!`.
