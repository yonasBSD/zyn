I've been writing proc macros for a while now. Derive macros for internal tools, attribute macros for instrumentation. And every time, the same two problems: `quote!` doesn't compose (you end up passing `TokenStream` fragments through five layers of helper functions and writing hundreds of `let` statements), and debugging generated code means `cargo expand` and then squinting at unformatted token output hoping something jumps out.

Because of this I ended up writing the same helper methods, composite AST parsing and tokenizing types, extractors etc. I would have to copy these from project to project as needed, and eventually just decided to publish a crate so I never have to do it again.

So I built [zyn](https://github.com/aacebo/zyn) — a proc macro framework with a template language, composable components, and compile-time diagnostics.

## 🎯 Goals

1. Template syntax that supports expressions, looping, composition of reusable custom elements, and editor syntax highlighting + type safety.
2. Automated attribute arguments parsing.
3. Diagnostic pattern that supports more than just hard compiler errors and can emit more than one at a time, linked to the span it originated from. Ideally with editor integration.
4. Extensions for `syn` AST types to make querying the parsed AST easier.
5. Testing features like `debug` and assertion macros so I don't have to use `cargo expand` or stringify token streams and make fuzzy assertions.
6. **Comparable performance to using `syn` + `quote`** ([benchmarks](#-performance))

## 🔨 Building a [Builder](https://github.com/aacebo/zyn/tree/main/examples/builder)

I'm going to build a `#[derive(Builder)]` macro with it, start to finish. The whole thing comes out to about **60** lines.

[Features](https://docs.rs/zyn/latest/zyn/index.html#features)

```sh
cargo add zyn
```

What we want the user to write:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/tests/builder.rs#L3-L11) | [docs](https://aacebo.github.io/zyn/06-macros/derive.html)

```rust
#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    #[builder(default)]
    verbose: bool,
    #[builder(default_value = "30")]
    timeout: i64,
}
```

> ℹ️ A struct annotated with `#[derive(Builder)]`. Fields marked `#[builder(default)]` use `Default::default()` when omitted, and `#[builder(default_value = "...")]` uses a custom expression.

And what we want to generate:

```rust
struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    verbose: Option<bool>,
    timeout: Option<i64>,
}

impl ConfigBuilder {
    fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }
    // ... setters for each field ...

    fn build(self) -> Config {
        Config {
            host: self.host.expect("field `host` is required"),
            port: self.port.expect("field `port` is required"),
            verbose: self.verbose.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_else(|| 30),
        }
    }
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder {
            host: None,
            port: None,
            verbose: None,
            timeout: None,
        }
    }
}
```

With raw `quote!`, this gets messy fast — nested iterations, conditional logic for defaults, splicing field names and types everywhere.

## 🏷️ Typed Attribute Parsing

First, parsing `#[builder(default)]` and `#[builder(default = expr)]`. Doing this by hand means a `syn::parse::Parse` impl, handling every variant, producing decent errors. With zyn:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L3-L11) | [docs](https://aacebo.github.io/zyn/04-attributes/)

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default)]
    default: bool,
    default_value: Option<String>,
}
```

> ℹ️ Declares a typed config for `#[builder(...)]` attributes. `#[zyn("builder")]` sets the attribute name to match. `#[zyn(default)]` fields default to `false`/`None` when omitted. zyn generates `from_args()` and `from_input()` parsing methods automatically.

That generates `from_args()` and `from_input()` methods. We add a convenience `from_field` that extracts from a field's attributes using the `ext` feature:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L1-L29) | [docs](https://aacebo.github.io/zyn/04-attributes/)

```rust
use zyn::ext::AttrExt;

impl BuilderConfig {
    fn from_field(field: &zyn::syn::Field) -> Self {
        let attr = field.attrs.iter().find(|a| a.is("builder"));

        match attr {
            Some(attr) => {
                let args = attr.args().unwrap();
                Self::from_args(&args).unwrap()
            }
            None => Self {
                skip: false,
                default: false,
                default_value: None,
            },
        }
    }
}
```

> ℹ️ A convenience method that extracts `BuilderConfig` from a field's attributes. `AttrExt::is()` finds the `#[builder(...)]` attribute, `args()` parses its arguments, and `from_args()` maps them into the typed struct. Returns defaults when no attribute is present.

Typo suggestions come free 💡:

```
error: unknown argument `skiip`
  |
5 | #[builder(skiip)]
  |           ^^^^^
  |
  = help: did you mean `skip`?
```

Levenshtein distance. Your users get `did you mean skip?` instead of `unexpected token`.

## 🧩 Composable Elements

Instead of one giant `quote!` block, you break the macro into **elements** — reusable template components with typed props.

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L31-L39) | [docs](https://aacebo.github.io/zyn/03-elements/)

```rust
#[zyn::element]
fn setter(
    name: zyn::syn::Ident,
    ty: zyn::syn::Type,
) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}
```

> ℹ️ An element that generates a builder setter method. Takes a field name and type as props, produces a method that sets the corresponding `Option` field and returns `self` for chaining.

If you wanted methods like `with_host` instead of `host`, pipes handle it inline: `{{ name | ident:"with_{}" }}`. They compose — `{{ name | upper | ident:"SET_{}" }}` would produce `SET_HOST` from `host`.

The build method, where defaults come in:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L41-L55) | [docs](https://aacebo.github.io/zyn/03-elements/)

```rust
#[zyn::element]
fn build_field(
    name: zyn::syn::Ident,
    config: BuilderConfig,
) -> zyn::TokenStream {
    let name_str = name.to_string();

    if config.default {
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_default())
    } else if let Some(ref expr) = config.default_value {
        let default_expr: zyn::syn::Expr = zyn::syn::parse_str(expr).unwrap();
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_else(|| {{ default_expr }}))
    } else {
        zyn::zyn!({{ name }}: self.{{ name }}.expect(
            ::std::concat!("field `", {{ name_str | str }}, "` is required")
        ))
    }
}
```

> ℹ️ Generates a single field assignment inside `build()`. Uses `unwrap_or_default()` for `#[builder(default)]`, `unwrap_or_else(|| expr)` for `#[builder(default_value)]`, and panics with a descriptive message for required fields.

The setter doesn't care about defaults — that's `build_field`'s job.

## ⚙️ The Derive Entry Point

The derive uses **extractors** — typed parameters that zyn resolves from the macro input automatically:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L57-L121) | [docs](https://aacebo.github.io/zyn/06-macros/derive.html)

```rust
#[zyn::derive("Builder", attributes(builder))]
fn builder(
    #[zyn(input)] ident: zyn::syn::Ident,
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::TokenStream {

    zyn::zyn! {
        struct {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                @setter(
                    name = field.ident.clone().unwrap(),
                    ty = field.ty.clone(),
                )
            }

            fn build(self) -> {{ ident }} {
                {{ ident }} {
                    @for (field in fields.named.iter()) {
                        @build_field(
                            name = field.ident.clone().unwrap(),
                            config = BuilderConfig::from_field(field),
                        ),
                    }
                }
            }
        }

        impl {{ ident }} {
            fn builder() -> {{ ident | ident:"{}Builder" }} {
                {{ ident | ident:"{}Builder" }} {
                    @for (field in fields.named.iter()) {
                        {{ field.ident }}: None,
                    }
                }
            }
        }
    }
}
```

> ℹ️ The derive entry point. Extractors (`#[zyn(input)]`) resolve `ident` and `fields` from the derive input automatically. The template generates a `FooBuilder` struct with `Option`-wrapped fields, setter methods via `@setter`, a `build()` method that unwraps each field via `@build_field`, and a `Foo::builder()` constructor.

Parameters marked `#[zyn(input)]` are extractors — `ident` gets resolved from the derive input automatically, `Fields<FieldsNamed>` pulls the named fields. If someone puts `#[derive(Builder)]` on an enum, zyn emits a compile error automatically.

The `@for` loops iterate fields. The `@setter` and `@build_field` calls compose the pieces. The template reads top-to-bottom as one block, no splicing iterator chains back together like you would with `quote!`.

## 🩺 Diagnostics

Standard proc macros bail on the first error. Fix, recompile, hit the next one.

zyn accumulates them. Add some validation to the builder:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/src/lib.rs#L62-L82) | [docs](https://aacebo.github.io/zyn/03-elements/diagnostics.html)

```rust
for field in fields.named.iter() {
    let config = BuilderConfig::from_field(field);

    if config.skip && config.default {
        error!(
            "`skip` and `default` are mutually exclusive on field `{}`",
            field.ident.as_ref().unwrap();
            span = field.ident.as_ref().unwrap().span()
        );
    }

    if config.skip && config.default_value.is_some() {
        warn!(
            "`default_value` is ignored when `skip` is set";
            span = field.ident.as_ref().unwrap().span()
        );
    }
}

// stop here if any errors accumulated, otherwise continue to codegen
bail!();
```

> ℹ️ Validates field configurations before codegen. `error!` and `warn!` accumulate diagnostics with span information instead of panicking on the first problem. `bail!()` stops compilation only if errors were recorded — warnings pass through.

`error!`, `warn!`, `note!`, `help!` are injected into every `#[zyn::derive]`, `#[zyn::element]`, and `#[zyn::attribute]` body. `bail!()` with no arguments checks if any errors were accumulated and returns early — but only if there are errors. Warnings pass through.

Users see every problem in one compile pass. ✅

## 🔍 Debugging

I wrote the debug system after spending two days on a bug where a generated impl block was missing a lifetime bound. `cargo expand` spat out 400 lines of tokens and I couldn't find it, so I built a debug system.

Add `debug = "pretty"` to any element, derive, or attribute macro:

[docs](https://aacebo.github.io/zyn/07-testing/debugging.html)

```rust
#[zyn::element(debug = "pretty")]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    // ...
}
```

```sh
ZYN_DEBUG="Setter" cargo build
```

> ℹ️ Opts the element into debug output. `debug = "pretty"` formats the generated code through `prettyplease`. The `ZYN_DEBUG` env var controls which macros emit output — wildcard patterns like `"*"` match everything.

![debug pretty](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/examples/builder/assets/screenshot-1.png)

Generated code shows up as a compiler note — in your terminal, in your IDE's Problems panel. `pretty` runs it through `prettyplease` so you get formatted Rust instead of token soup. Wildcard patterns work: `ZYN_DEBUG="*"` dumps everything.

## 🧪 Testing

zyn's test module gives you assertion macros that compare token streams structurally. Here's how we test the `setter` element from the builder:

[source](https://github.com/aacebo/zyn/blob/main/examples/builder/tests/elements.rs) | [docs](https://aacebo.github.io/zyn/07-testing/assertions.html)

```rust
use zyn::quote::quote;

#[zyn::element(debug = "pretty")]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}

#[test]
fn setter_generates_expected_signature() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("port"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("u16").unwrap(),
        )
    );
    let expected = quote! {
        fn port(mut self, value: u16) -> Self {
            self.port = Some(value);
            self
        }
    };

    zyn::assert_tokens!(output, expected);
}

#[test]
fn setter_pretty_output() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("host"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("String").unwrap(),
        )
    );

    zyn::assert_tokens_contain_pretty!(output, "fn host(mut self, value: String) -> Self");
}
```

> ℹ️ Tests for the `setter` element. `assert_tokens!` compares token streams structurally — no whitespace sensitivity. `assert_tokens_contain_pretty!` does substring matching on formatted output for readable assertions.

`assert_tokens!` compares structurally — no `to_string()` comparisons that break on whitespace. `assert_tokens_contain!` does substring matching on the cleaned output. `assert_tokens_contain_pretty!` (behind the `pretty` feature) gives you human-readable diffs when things fail.

## ⚡ Performance

Benchmarks are run via CI on push and also on a schedule.

> The full pipeline (parse → extract → codegen) compared to equivalent hand-written `syn` + `quote!`:

<a href="https://bencher.dev/perf/zyn?lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=d618e093-bbbc-439f-82af-4502c72cd2bd&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&benchmarks=19886ff1-468f-4126-a1a8-01b680c66df3,bc8919bf-3786-4119-8232-c86165c96c50&measures=f051294e-7710-4809-a4b7-1181628e464b&tab=plots&key=true&title=Full%20Pipeline&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=zyn"><img src="https://api.bencher.dev/v0/projects/zyn/perf/img?branches=d618e093-bbbc-439f-82af-4502c72cd2bd&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&benchmarks=19886ff1-468f-4126-a1a8-01b680c66df3,bc8919bf-3786-4119-8232-c86165c96c50&measures=f051294e-7710-4809-a4b7-1181628e464b&title=Full%20Pipeline" title="Full Pipeline" alt="Full Pipeline - Bencher" /></a>

[more benchmarks](https://github.com/aacebo/zyn/blob/main/BENCH.md).

## 🚀 Try It

```sh
cargo add zyn
```

There's also extension traits behind the `ext` feature for common `syn` operations — `field.is_option()`, `attr.exists("builder")`, keyed field access. Saves some repetitive `syn` traversal.

The [getting started guide](https://aacebo.github.io/zyn) walks through everything. The [API docs](https://docs.rs/zyn) cover every type and trait. The full [builder example](https://github.com/aacebo/zyn/tree/main/examples/builder) from this post is in the repo with tests.

I built zyn because `quote!` was making me miserable. It's not done — there are rough edges around macro hygiene in some edge cases — but it's how I write every proc macro now.

🔗 [GitHub](https://github.com/aacebo/zyn) | [crates.io](https://crates.io/crates/zyn) | [Docs](https://docs.rs/zyn)
