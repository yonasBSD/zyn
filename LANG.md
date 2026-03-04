# zyn Language Reference

zyn is a proc-macro template DSL for Rust. It compiles at macro-expansion time into code that builds a `proc_macro2::TokenStream`. The input to `zyn!(...)` is a stream of template nodes; the output is a Rust block expression evaluating to a `TokenStream`.

---

## 1. Grammar

```ebnf
Element     = Node*

Node        = TokensNode
            | InterpNode
            | GroupNode
            | AtNode

TokensNode  = token_tree+
            (* any token tree(s) that are NOT: '@', '{', '(', '[' *)

InterpNode  = '{' '{' Expr ('|' Pipe)* '}' '}'
            (* outer brace content must be exactly one inner brace group *)

Expr        = token_tree+
            (* all token trees before the first '|' or the closing inner '}' *)

Pipe        = ident (':' PipeArg)*
PipeArg     = token_tree+
            (* tokens until the next ':' or '|' *)

GroupNode   = '(' Element ')'
            | '[' Element ']'
            | '{' Element '}'    (* only when brace disambiguation fails *)

AtNode      = '@' 'if'    IfBody
            | '@' 'for'   ForBody
            | '@' 'match' MatchBody
            | '@' 'throw' LitStr
            | '@' ElementName ElementBody

IfBody      = '(' Expr ')' '{' Element '}' ElseClause*
ElseClause  = '@' 'else' 'if' '(' Expr ')' '{' Element '}'
            | '@' 'else' '{' Element '}'

ForBody     = '(' ident 'of' Expr ')' '{' Element '}'

MatchBody   = '(' Expr ')' '{' MatchArm* '}'
MatchArm    = Pattern '=>' '{' Element '}' ','?
Pattern     = token_tree+    (* all tokens before '=>' *)

ElementName = ident ('::' ident)*
ElementBody = ('(' Props ')')? ('{' Element '}')?
Props       = PropField (',' PropField)* ','?
PropField   = ident '=' PropValue
PropValue   = token_tree+    (* all tokens until ',' or ')' *)
```

---

## 2. Parsing Model

`Element::parse` is the root parser. It reads token trees in a loop, dispatching on the leading token:

| Leading token | Action |
|---|---|
| `@` | Flush pending tokens; parse `AtNode` |
| `{` | Flush pending tokens; run brace disambiguation |
| `(` | Flush pending tokens; parse `GroupNode` (Parenthesis) |
| `[` | Flush pending tokens; parse `GroupNode` (Bracket) |
| anything else | Accumulate into pending `TokenStream` |

When the input is exhausted, any pending tokens are flushed as a `TokensNode`.

---

## 3. Brace Disambiguation

When the parser sees `{`, it must decide between an `InterpNode` and a brace-delimited `GroupNode`. The algorithm:

1. Consume the outer `{`, capturing its content stream.
2. **Fork** the content stream and attempt to consume a single inner `{` group.
3. If the fork consumed exactly one inner brace group AND the outer content is now empty, it is an **interpolation** (`InterpNode`).
4. Otherwise, the outer brace is a **group** (`GroupNode` with `Delimiter::Brace`).

In concrete terms:
- `{{ expr }}` — outer brace contains only `{ expr }` → `InterpNode`
- `{ anything }` where `anything` is not a single inner brace group → `GroupNode`
- `{{ }}` (empty inner) → parse error: `"empty interpolation"`

The outer and inner braces are written adjacently in source: `{{` is two separate `{` tokens, not a special lexeme.

---

## 4. Token Passthrough (`TokensNode`)

Any token tree that is not `@`, `{`, `(`, or `[` is accumulated into a pending buffer. When a special form is encountered (or the input ends), the buffer is flushed as a `TokensNode`.

A `TokensNode` expands to:
```rust
output.extend(::quote::quote!( <stream> ));
```

This means any valid Rust token sequence passes through verbatim. Multiple consecutive passthrough tokens are batched into a single node.

---

## 5. Interpolation (`InterpNode`)

```
{{ expr }}
{{ expr | pipe }}
{{ expr | pipe | pipe }}
{{ expr | pipe:arg }}
{{ expr | pipe:arg:arg }}
```

### Expression

`expr` is accumulated as raw token trees until the first `|` token or the closing inner `}`. It can be:
- A simple identifier: `{{ name }}`
- A field path: `{{ item.field.name }}`
- A method call: `{{ names.len() }}`
- Any Rust expression that produces a value implementing `ToTokens` or `ToString`

### Without pipes

Expands to:
```rust
::quote::ToTokens::to_tokens(&( expr ), &mut output);
```

The value must implement `ToTokens`.

### With pipes

The expression is **stringified** first, then each pipe transforms `__zyn_val`:

```rust
{
    let __zyn_val = ( expr ).to_string();
    // pipe 1:
    let __zyn_val = ::zyn::Pipe::pipe(&( Pipe1 ), __zyn_val);
    // between pipes, re-stringify:
    let __zyn_val = __zyn_val.to_string();
    // pipe 2:
    let __zyn_val = ::zyn::Pipe::pipe(&( Pipe2 ), __zyn_val);
    ::quote::ToTokens::to_tokens(&__zyn_val, &mut output);
}
```

The re-stringify step (`let __zyn_val = __zyn_val.to_string()`) happens between every pipe except before the first. This means each pipe in a chain receives a `String` regardless of what the previous pipe returned.

---

## 6. Pipes

### Syntax

```
pipe_name
pipe_name:arg1
pipe_name:arg1:arg2
```

The pipe name is an identifier. Arguments are delimited by `:` and each argument is a token stream running until the next `:` or `|`. Arguments become constructor parameters to the pipe struct.

### Builtin pipes

The following pipe names are recognized as builtins and resolved as `::zyn::PipeName`:

| Name | Args | Input | Output | Behavior |
|------|------|-------|--------|----------|
| `upper` | — | `String` | `Ident` | `input.to_uppercase()` |
| `lower` | — | `String` | `Ident` | `input.to_lowercase()` |
| `snake` | — | `String` | `Ident` | convert to `snake_case` |
| `camel` | — | `String` | `Ident` | convert to `camelCase` |
| `pascal` | — | `String` | `Ident` | convert to `PascalCase` |
| `kebab` | — | `String` | `LitStr` | convert to `"kebab-case"` (string literal, not `Ident`, because `-` is not valid in identifiers) |
| `screaming` | — | `String` | `Ident` | convert to `SCREAMING_SNAKE_CASE` |
| `ident` | pattern | `String` | `Ident` | replace `{}` in pattern with input; result is an `Ident` |
| `fmt` | pattern | `String` | `LitStr` | replace `{}` in pattern with input; result is a string literal |

For builtin pipes without args the expansion is:
```rust
let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::Pascal), __zyn_val);
```

For builtin pipes with args (`ident`, `fmt`):
```rust
let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::Ident("get_{}")), __zyn_val);
```

### Custom pipes

Any pipe name not in the builtin list is treated as a custom pipe. The name is converted to **PascalCase** and resolved as a path in the caller's scope:

```rust
// {{ name | shout }}  →
let __zyn_val = ::zyn::Pipe::pipe(&(Shout), __zyn_val);

// {{ name | shout:arg }}  →
let __zyn_val = ::zyn::Pipe::pipe(&(Shout(arg)), __zyn_val);
```

Custom pipes must implement `::zyn::Pipe`. See [Section 10](#10-defining-pipes-pipe) for how to declare them.

### Pipe chaining

Pipes execute left to right. Between each pipe the previous output is stringified via `.to_string()`, so every pipe in a chain receives a `String` as input regardless of what the previous pipe returned.

```
{{ name | snake | ident:"get_{}" }}
```
Expands to:
```rust
{
    let __zyn_val = (name).to_string();
    let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::Snake), __zyn_val);
    let __zyn_val = __zyn_val.to_string();
    let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::Ident("get_{}")), __zyn_val);
    ::quote::ToTokens::to_tokens(&__zyn_val, &mut output);
}
```

---

## 7. Groups (`GroupNode`)

```
( Element )
[ Element ]
{ Element }
```

A group wraps an `Element` in a delimiter. The inner `Element` is parsed recursively. Groups are used to produce grouped token trees in the output (e.g. function argument lists, array brackets, bare brace blocks).

Expansion:
```rust
{
    let mut __zyn_ts_N = ::proc_macro2::TokenStream::new();
    // expanded inner Element writing to __zyn_ts_N
    ::quote::ToTokens::to_tokens(
        &::proc_macro2::Group::new(::proc_macro2::Delimiter::Parenthesis, __zyn_ts_N),
        &mut output,
    );
}
```

A new unique identifier (`__zyn_ts_N`) is allocated for each nested group. The delimiter reflects the source: `Parenthesis`, `Bracket`, or `Brace`.

---

## 8. Directives (`AtNode`)

All directives begin with `@`. The `@` token is consumed first, then a plain identifier (using `parse_any` to allow keywords like `if`, `for`, `match`). The identifier determines the directive kind.

`@else` appearing without a preceding `@if` is always a parse error.

### 8.1 `@if`

```
@if ( expr ) { body }
@if ( expr ) { body } @else if ( expr ) { body }
@if ( expr ) { body } @else { body }
@if ( expr ) { body } @else if ( expr ) { body } @else { body }
```

- `expr` is any Rust expression (raw token stream).
- `body` is an `Element` parsed inside a braced block.
- `@else if` and `@else` clauses are consumed greedily after the closing `}` of the previous branch. They must immediately follow (no intervening tokens).
- Any number of `@else if` branches may appear; at most one `@else` branch at the end.

**Detection of `@else`**: the parser peeks ahead — if the next tokens are `@` followed by the identifier `else`, it consumes them. If `else` is followed by the identifier `if`, it becomes `@else if`; otherwise it is a bare `@else`.

Expands to native Rust:
```rust
if expr { body_expansion }
else if expr { body_expansion }
else { body_expansion }
```

### 8.2 `@for`

```
@for ( binding of iter ) { body }
```

- `binding` is a single identifier.
- The keyword between binding and iter **must** be `of`. Using `in` is a compile error with the diagnostic: `"expected 'of' ('in' is a Rust keyword, use 'of' instead)"`. Any other token is a compile error: `"expected 'of'"`.
- `iter` is any Rust expression (raw token stream, consumes the rest of the parens).
- `body` is an `Element`.

Expands to:
```rust
for binding in iter {
    body_expansion
}
```

### 8.3 `@match`

```
@match ( expr ) {
    pattern => { body }
    pattern => { body },
}
```

- `expr` is any Rust expression.
- Arms consist of a pattern (all tokens before `=>`), then a braced body `Element`.
- Trailing `,` after each arm body is optional.
- Wildcard `_` and string/integer literal patterns work directly (they are raw token streams).

Expands to:
```rust
match expr {
    pattern => { body_expansion },
    pattern => { body_expansion },
}
```

### 8.4 `@throw`

```
@throw "error message"
```

- The argument must be a **string literal** (`LitStr`). Non-literal tokens cause a parse error.
- Expands to a compile error at the source span of the message:

```rust
::core::compile_error!("error message");
```

This is a hard compile-time error in any code path that reaches it.

### 8.5 `@element` (custom elements)

```
@name
@name()
@name(prop = value)
@name(prop = value, prop = value)
@name { children }
@name() { children }
@name(prop = value) { children }
@module::name(prop = value) { children }
```

- `name` is any identifier not reserved as a built-in directive (`if`, `for`, `match`, `throw`, `else`).
- The name may be a `::` separated path (e.g. `@components::field_decl`). Only the **last** segment is PascalCased for struct resolution; intermediate path segments pass through unchanged.
- Props are `ident = value` pairs separated by `,`. The value is accumulated as token trees until the next `,` or `)`.
- The props parens `(...)` are optional. Omitting them is equivalent to empty props.
- The children block `{ ... }` is optional. If present, its content is parsed as an `Element` and passed as the `children: TokenStream` field.
- The element name is **automatically converted to PascalCase** at expansion time. `@greeting` → `Greeting`, `@field_decl` → `FieldDecl`, `@components::field_decl` → `components::FieldDecl`.

Expands to (without children):
```rust
{
    let __zyn_rendered = ::zyn::Render::render(&Name {
        prop: value,
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut output);
}
```

Expands to (with children):
```rust
{
    let mut __zyn_ts_N = ::proc_macro2::TokenStream::new();
    // children Element expanded into __zyn_ts_N
    let __zyn_rendered = ::zyn::Render::render(&Name {
        prop: value,
        children: __zyn_ts_N,
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut output);
}
```

The `?` means element expansion can return a `syn::Result` error, which propagates out of the enclosing function. The caller must be in a context returning `syn::Result<_>`.

---

## 9. Expansion Model

Every `zyn!(...)` invocation expands to a single Rust **block expression** that evaluates to `proc_macro2::TokenStream`:

```rust
{
    let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
    // ... expanded nodes writing to __zyn_ts_0 ...
    __zyn_ts_0
}
```

### Variable naming

An incrementing counter allocates unique identifiers:

| Identifier | Purpose |
|---|---|
| `__zyn_ts_0` | Root output accumulator |
| `__zyn_ts_1`, `__zyn_ts_2`, … | Inner accumulators for groups and element children |
| `__zyn_val` | Pipe chain intermediate (always this name; scoped inside `{}`) |
| `__zyn_rendered` | Result of `Render::render(...)` inside an element call |

Nested structures (groups inside groups, elements inside loops) each get a fresh `__zyn_ts_N` at the level where they are expanded, allocated sequentially from the same counter.

### Control flow

`@if`, `@for`, and `@match` expand to native Rust control flow that writes directly to the enclosing output accumulator. There is no intermediate collection; nodes inside loop bodies append to `output` on each iteration.

---

## 10. Defining Elements (`#[element]`)

The `#[zyn::element]` attribute macro transforms a function into a struct implementing `::zyn::Render`.

### Input

```rust
#[zyn::element]
fn element_name(
    param1: Type1,
    param2: Type2,
    children: proc_macro2::TokenStream,  // optional
) -> syn::Result<proc_macro2::TokenStream> {
    // body
}
```

### Output

```rust
pub struct ElementName {
    pub param1: Type1,
    pub param2: Type2,
    pub children: proc_macro2::TokenStream,
}

impl ::zyn::Render for ElementName {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
        let param1 = &self.param1;
        let param2 = &self.param2;
        let children = &self.children;
        // original body
    }
}
```

The function name is converted to PascalCase for the struct name. Each parameter becomes a public field. In the `render` body, each field is re-bound as a local let binding so the original body code works without `self.` prefixes.

### Custom name

```rust
#[zyn::element("alias_name")]
fn implementation_name(...) -> syn::Result<proc_macro2::TokenStream> { ... }
```

This additionally generates:
```rust
use ImplementationName as AliasName;
```

The template then uses `@alias_name(...)` (which resolves to `AliasName` via PascalCase conversion).

### `children` field

If the function has a parameter named `children` of type `proc_macro2::TokenStream`, the element accepts a body block in the template (`@name(...) { ... }`). The expanded children `TokenStream` is passed as this field.

Elements without a `children` parameter do not accept a body block in the template.

### Zero-parameter elements

An element with no parameters may be invoked without parens:
```
@divider
@divider()
```
Both forms are valid.

---

## 11. Defining Pipes (`#[pipe]`)

The `#[zyn::pipe]` attribute macro transforms a function into a struct implementing `::zyn::Pipe`.

### Input

```rust
#[zyn::pipe]
fn pipe_name(input: InputType) -> OutputType {
    // body
}
```

### Output

```rust
pub struct PipeName;

impl ::zyn::Pipe for PipeName {
    type Input = InputType;
    type Output = OutputType;

    fn pipe(&self, input: InputType) -> OutputType {
        // original body
    }
}
```

### Custom name

```rust
#[zyn::pipe("alias_name")]
fn implementation_name(input: InputType) -> OutputType { ... }
```

Generates:
```rust
use ImplementationName as AliasName;
```

The template uses `{{ expr | alias_name }}`.

### Pipe input convention

All builtin pipes accept `String` as input. Custom pipes in a chain also receive `String` because the intermediate value is stringified between each pipe (via `.to_string()`). Custom pipes should therefore declare `input: String` unless they are always the first pipe in a chain and receive a non-String `ToTokens` value directly.

---

## 12. Case Conversion Rules

The following conversion rules are used both by pipes and internally (e.g. PascalCasing element names).

### `to_snake(s)`

Inserts `_` at case-transition boundaries in the input, then lowercases everything:

- Before an uppercase letter preceded by a lowercase letter: `"helloWorld"` → `"hello_world"`
- Before an uppercase letter preceded by another uppercase and followed by a lowercase: `"XMLParser"` → `"x_m_l_parser"` (note: transitions in acronyms)
- Existing `_` are preserved (collapsed if already at start or after another `_`)

### `to_pascal(s)`

1. Apply `to_snake(s)` to normalize to a common base form.
2. Capitalize the first character of each `_`-separated word; remove the `_`.

Examples: `"hello_world"` → `"HelloWorld"`, `"camelCase"` → `"Camelcase"` (via snake: `"camel_case"` → `"CamelCase"`).

### `to_camel(s)`

Apply `to_pascal(s)` then lowercase the first character.

### `to_screaming(s)`

Apply `to_snake(s)` then `to_uppercase()`.

### `to_kebab(s)`

Apply `to_snake(s)` then replace every `_` with `-`. Result is a string literal (`LitStr`), not an `Ident`, because `-` is not valid in Rust identifiers.

### Path PascalCasing (element names)

For a `::` separated path like `components::field_decl`, only the **last** segment is converted to PascalCase. All preceding segments and `::` separators are emitted verbatim. This is implemented by scanning the token stream for the last `Ident` token.

---

## 13. Error Cases

| Situation | Error |
|---|---|
| `{{ }}` (empty interpolation) | parse error: `"empty interpolation"` |
| `@else` not preceded by `@if` | parse error: `"unexpected @else without @if"` |
| `@for (x in ...)` | parse error: `"expected 'of' ('in' is a Rust keyword, use 'of' instead)"` |
| `@for (x foo ...)` | parse error: `"expected 'of'"` |
| `@throw expr` where `expr` is not a string literal | parse error from `syn::LitStr` parsing |
| Element `render` returning `Err` | runtime error propagated via `?` at expansion time |
| Pipe output not implementing `ToTokens` | Rust type error in generated code |

---

## 14. Trait Reference

### `Render`

```rust
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}
```

Implemented by types created with `#[element]`. Called inside the `zyn!(...)` expansion for every `@element` node.

### `Pipe`

```rust
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
```

Implemented by types created with `#[pipe]` and by all builtin pipe structs. `Output` must implement `ToTokens` so the final result can be emitted to the token stream.

### `Expand` (internal)

```rust
pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}
```

Internal trait implemented by every AST node. `output` is the name of the current accumulator variable. `idents` is a shared counter used to generate unique `__zyn_ts_N` names. Not part of the public API surface; relevant only when implementing new node types.
