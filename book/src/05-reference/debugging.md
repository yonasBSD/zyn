# Debugging with `debug!`

`zyn::debug!` is a drop-in replacement for `zyn!` that prints what the template produces, then returns the same tokens. Use it to see exactly what code your template generates.

## Basic Usage

```rust
let tokens = zyn::debug! {
    struct {{ name }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
};
```

```bash
zyn::debug! ─── pretty
struct MyStruct {
    name: String,
    age: u32,
}
```

## Modes

Specify a mode before `=>` to control the output format:

### `pretty` (default)

Shows the **final Rust code** your template produces — the actual output, formatted with indentation.

```rust
zyn::debug! { pretty =>
    @if (is_pub) { pub }
    fn {{ name | snake }}() {}
}
```

When no mode is specified, `pretty` is used:

```rust
zyn::debug! {
    fn {{ name }}() {}
}
```

The output appears on `stderr` at runtime (when the proc macro executes), so it's visible in `cargo build` and `cargo test` output.

### `raw`

Shows the **expansion code** — the token-building machinery that `zyn!` generates behind the scenes. Emitted as a compile-time diagnostic (zero runtime cost).

```rust
zyn::debug! { raw =>
    struct {{ name }} {}
}
```

```bash
note: zyn::debug! ─── raw

{
    let mut output = TokenStream::new();
    output.extend(quote!(struct));
    ToTokens::to_tokens(&(name), &mut output);
    output.extend(quote!({}));
    output
}
```

The output is cleaned up for readability — `__zyn_ts_0` becomes `output`, fully-qualified paths are simplified.

### `ast`

Shows the **parsed template structure** — which AST nodes the parser created. Emitted as a compile-time diagnostic.

```rust
zyn::debug! { ast =>
    @if (is_pub) { pub }
    struct {{ name }} {}
}
```

```bash
note: zyn::debug! ─── ast

Template [
  At(If)
  Tokens("struct")
  Interp { ... }
  Tokens("{}")
]
```
