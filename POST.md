# `zyn` – a template engine and framework for writing procedural macros

I've been working on [`zyn`](https://github.com/aacebo/zyn), a framework that tries to make writing proc macros less painful. Instead of assembling everything out of raw `quote!` blocks and manual attribute parsing, `zyn` gives you a small template DSL, reusable code-generation components, and typed attribute extraction.

It's at v0.2.0 and published on crates.io.

---

## The problem

Writing a non-trivial proc macro typically means:

- Deeply nested `quote!` blocks that become hard to follow
- Re-implementing attribute parsing boilerplate for every crate
- Duplicated string transformation logic scattered across `format_ident!` calls
- No real way to break code generation into reusable pieces

---

## What `zyn` provides

### Templates with interpolation and pipes

Instead of `quote! { fn #snake_name() {} }`, you write:

```rust
zyn! {
    fn {{ name | snake }}() {}
}
```

Pipes transform values inline — `snake`, `camel`, `pascal`, `screaming`, `kebab`, `upper`, `lower`, `str`, `trim:"_"`, `plural`, `singular`, `ident:"get_{}"`, `fmt:"{}!"` — and they compose:

```rust
zyn! {
    const {{ name | snake | upper }}: &str = {{ name | fmt:"{}!" }};
}
```

### Control flow

```rust
zyn! {
    @if (input.is_pub) { pub }
    @else { pub(crate) }

    struct {{ input.ident }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
```

### Elements — reusable code-generation components

This is the part I'm most excited about. `#[zyn::element]` lets you define parameterized, composable pieces of code generation and call them from templates:

```rust
#[zyn::element]
fn field_decl(
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
) -> zyn::TokenStream {
    zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
}

zyn! {
    struct {{ input.ident }} {
        @for (field in fields.iter()) {
            @field_decl(
                vis = field.vis.clone(),
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

Elements can also accept children blocks, which makes wrapping patterns clean:

```rust
#[zyn::element]
fn wrapper(vis: syn::Visibility, children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::zyn! { {{ vis }} mod generated { {{ children }} } }
}

zyn! {
    @wrapper(vis = input.vis.clone()) {
        pub fn hello() {}
    }
}
```

### Typed attribute parsing

`#[derive(zyn::Attribute)]` generates a typed parser for your macro's attributes:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default = "build".to_string())]
    method: String,
    skip: bool,
}

// In your proc macro:
let cfg = BuilderConfig::from_input(&input)?;
```

Fields can be auto-resolved in elements without passing them explicitly:

```rust
#[zyn::element]
fn builder_method(
    #[zyn(input)] cfg: zyn::Attr<BuilderConfig>,
    name: syn::Ident,
) -> zyn::TokenStream {
    let method = zyn::format_ident!("{}", cfg.method);
    zyn::zyn! { pub fn {{ method }}(self) -> Self { self } }
}
```

### Custom pipes

```rust
#[zyn::pipe]
fn prefix(input: String) -> syn::Ident {
    syn::Ident::new(&format!("pfx_{}", input), zyn::Span::call_site())
}

zyn! { {{ name | prefix }} }
```

---

## How it relates to existing crates

- **`quote!`** — `zyn` uses `quote` internally and the `zyn!` macro compiles down to the same token stream manipulation. You can mix `zyn!` and `quote!` freely.
- **`darling`** — `#[derive(zyn::Attribute)]` covers similar ground. `darling` is more mature; `zyn`'s version integrates directly with the element system.
- **`proc-macro-error`** — `zyn` has its own `Diagnostics` accumulator and `error!`/`warn!`/`bail!` macros generated into element bodies.

---

## Links

- `cargo add zyn`
- GitHub: https://github.com/aacebo/zyn
- Docs/book: https://aacebo.github.io/zyn

---

I'd love feedback, especially on the element composition model — it's the part most unlike anything else in the ecosystem and I'm curious whether it resonates with people who write complex proc macros.
