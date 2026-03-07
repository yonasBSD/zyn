# zyn — a template engine for Rust proc macros

I kept rebuilding the same proc macro scaffolding across my own crates — `syn` for parsing, `quote` for codegen, `heck` for case conversion, `proc-macro-error` for diagnostics, hand-rolled attribute parsing, and a pile of helper functions returning `TokenStream`. Every project was the same patchwork. zyn started as a way to stop repeating myself, and turned into a framework that replaces all of it with a single crate.

<a href="https://aacebo.github.io/zyn" target="_blank">
    <img src="https://img.shields.io/badge/📖 Getting Started-blue?style=for-the-badge" />
</a>
<a href="https://docs.rs/zyn" target="_blank">
    <img src="https://img.shields.io/docsrs/zyn" alt="docs.rs" />
</a>
<a href="https://crates.io/crates/zyn" target="_blank">
    <img src="https://img.shields.io/crates/v/zyn" alt="crates.io" />
</a>

`cargo add zyn`

## What it looks like

### Templates with control flow

With `quote!`, every conditional or loop forces you out of the template:

```rust
let fields_ts: Vec<_> = fields
    .iter()
    .map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty, }
    })
    .collect();

quote! {
    struct #ident {
        #(#fields_ts)*
    }
}
```

With zyn:

```rust
zyn! {
    struct {{ ident }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
// generates: struct User { name: String, age: u32, }
```

`@if`, `@for`, and `@match` all work inline. No `.iter().map().collect()`.

### Case conversion and formatting

Before:

```rust
use heck::ToSnakeCase;

let getter = format_ident!(
    "get_{}",
    name.to_string().to_snake_case()
);
```

After:

```rust
{{ name | snake | ident:"get_{}" }}
// HelloWorld -> get_hello_world
```

13 built-in pipes: `snake`, `camel`, `pascal`, `screaming`, `kebab`, `upper`, `lower`, `str`, `trim`, `plural`, `singular`, `ident`, `fmt`. They chain.

### Reusable components

`#[zyn::element]` turns a template into a callable component:

```rust
#[zyn::element]
fn getter(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}

zyn! {
    impl {{ ident }} {
        @for (field in fields.iter()) {
            @getter(
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
// generates:
// impl User {
//     pub fn get_name(&self) -> &String { &self.name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

Elements accept typed parameters, can receive children blocks, and compose with each other.

### Proc macro entry points

`#[zyn::derive]` and `#[zyn::attribute]` replace the raw `#[proc_macro_derive]` / `#[proc_macro_attribute]` annotations. Input is auto-parsed and extractors pull what you need:

```rust
#[zyn::derive]
fn my_getters(
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
    #[zyn(input)] fields: zyn::Fields,
) -> zyn::TokenStream {
    zyn::zyn! {
        impl {{ ident }} {
            @for (field in fields.iter()) {
                @getter(
                    name = field.ident.clone().unwrap(),
                    ty = field.ty.clone(),
                )
            }
        }
    }
}
```

Users write `#[derive(MyGetters)]` — the function name auto-converts to PascalCase:

```rust
#[derive(MyGetters)]
struct User {
    name: String,
    age: u32,
}

// generates:
// impl User {
//     pub fn get_name(&self) -> &String { &self.name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

### Diagnostics

`error!`, `warn!`, `note!`, `help!`, and `bail!` work inside `#[zyn::element]`, `#[zyn::derive]`, and `#[zyn::attribute]` bodies:

```rust
#[zyn::derive]
fn my_derive(
    #[zyn(input)] fields: zyn::Fields,
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
) -> zyn::TokenStream {
    if fields.is_empty() {
        bail!("at least one field is required");
    }

    zyn::zyn!(impl {{ ident }} {})
}
```

The compiler output:

```
error: at least one field is required
 --> src/main.rs:3:10
  |
3 | #[derive(MyDerive)]
  |          ^^^^^^^^
```

No `syn::Error` ceremony, no external crate for warnings.

### Typed attribute parsing

`#[derive(Attribute)]` generates a typed struct from helper attributes:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default = "build".to_string())]
    method: String,
}

#[zyn::derive("Builder", attributes(builder))]
fn builder(
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
    #[zyn(input)] fields: zyn::Fields,
    #[zyn(input)] cfg: zyn::Attr<BuilderConfig>,
) -> zyn::TokenStream {
    if cfg.skip {
        return zyn::zyn!();
    }

    let method = zyn::format_ident!("{}", cfg.method);
    zyn::zyn! {
        impl {{ ident }} {
            pub fn {{ method }}(self) -> Self { self }
        }
    }
}
```

`zyn::Attr<BuilderConfig>` auto-resolves from the input context — fields are parsed and defaulted automatically. Users write `#[builder(skip)]` or `#[builder(method = "create")]` on their structs.

## Full feature list

- `zyn!` template macro with `{{ }}` interpolation
- `@if` / `@for` / `@match` control flow
- 13 built-in pipes + custom pipes via `#[zyn::pipe]`
- `#[zyn::element]` — reusable template components with typed params and children
- `#[zyn::derive]` / `#[zyn::attribute]` — proc macro entry points with auto-parsed input
- Extractor system: `Extract<T>`, `Attr<T>`, `Fields`, `Variants`, `Data<T>`
- `error!`, `warn!`, `note!`, `help!`, `bail!` diagnostics
- `#[derive(Attribute)]` for typed attribute parsing
- `zyn::debug!` — drop-in `zyn!` replacement that prints expansions (`pretty`, `raw`, `ast` modes)
- Case conversion functions available outside templates (`zyn::case::to_snake()`, etc.)
- Re-exports `syn`, `quote`, and `proc-macro2` — one dependency in your `Cargo.toml`

## Benchmarks

<a href="https://bencher.dev/perf/zyn?key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1&report=872580c2-e3b1-493e-9aa2-bb78ba32b1b9&branches=d618e093-bbbc-439f-82af-4502c72cd2bd&heads=10a98bfb-f445-44d8-a7af-70c646b41b9e&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&benchmarks=a9077c45-a5ab-4fa8-8eea-5712bf3fa2dd%2Cb3e8cab4-ade2-4d10-9360-68543924b15c%2C41f60875-5055-48df-a59c-a4aa52aa2d85%2C9e76f3fb-c6c7-4825-a86a-f5574f6c0839%2C14f880fe-debb-4aa1-bc6f-e61b010e4aad%2Cf84312b5-0a93-4583-8cf0-34aecbb11289%2C0d1b6cd4-ddad-45c3-8024-98f531e45436%2Cf1682dba-4253-4336-b828-e4acd505254a%2C37568f5d-9b92-4dd8-971f-65a2d3c23efb%2C19886ff1-468f-4126-a1a8-01b680c66df3&measures=f051294e-7710-4809-a4b7-1181628e464b&start_time=1770321345000&end_time=1772913383000&lower_boundary=false&upper_boundary=false&clear=true&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=zyn"><img src="https://api.bencher.dev/v0/projects/zyn/perf/img?branches=d618e093-bbbc-439f-82af-4502c72cd2bd&heads=10a98bfb-f445-44d8-a7af-70c646b41b9e&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&specs=&benchmarks=a9077c45-a5ab-4fa8-8eea-5712bf3fa2dd%2Cb3e8cab4-ade2-4d10-9360-68543924b15c%2C41f60875-5055-48df-a59c-a4aa52aa2d85%2C9e76f3fb-c6c7-4825-a86a-f5574f6c0839%2C14f880fe-debb-4aa1-bc6f-e61b010e4aad%2Cf84312b5-0a93-4583-8cf0-34aecbb11289%2C0d1b6cd4-ddad-45c3-8024-98f531e45436%2Cf1682dba-4253-4336-b828-e4acd505254a%2C37568f5d-9b92-4dd8-971f-65a2d3c23efb%2C19886ff1-468f-4126-a1a8-01b680c66df3&measures=f051294e-7710-4809-a4b7-1181628e464b&start_time=1770321345000&end_time=1772913383000" title="zyn" alt="zyn - Bencher" /></a>

I'd appreciate any feedback — on the API design, the template syntax, the docs, or anything else. Happy to answer questions.

## License

MIT
