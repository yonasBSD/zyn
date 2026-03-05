# Phase 1: `Attribute` Trait

## Goal

Define the `Attribute` trait in `zyn-core` with built-in implementations for scalar types, containers, and `syn` types. Add typed accessor methods to `Arg`. No derive macro yet — that is Phase 2 and 3.

## The `Attribute` Trait

```rust
pub trait Attribute: Sized {
    fn from_args(args: &Args) -> syn::Result<Self>;
    fn from_arg(arg: &Arg) -> syn::Result<Self>;
    fn attribute(attrs: &[syn::Attribute]) -> syn::Result<Self>;
}
```

- `from_args` — extract from `&Args` (core extraction logic)
- `from_arg` — extract from a single `&Arg` (for nested `List` args and scalar values)
- `attribute` — find matching attribute(s) by name, parse args, call `from_args`

## Arg Enhancements

Add typed panicking accessor methods to `Arg`, following the existing `as_expr` / `as_args` / `as_lit` pattern:

| Method | Returns | Variant(s) |
|---|---|---|
| `as_flag(&self) -> &Ident` | The flag ident | `Flag` |
| `as_str(&self) -> String` | String value | `Expr` (string lit) or `Lit` (string) |
| `as_int<T: FromStr>(&self) -> T` | Parsed integer | `Expr` (int lit) or `Lit` (int) |
| `as_float<T: FromStr>(&self) -> T` | Parsed float | `Expr` (float lit) or `Lit` (float) |
| `as_char(&self) -> char` | Char value | `Expr` (char lit) or `Lit` (char) |
| `as_bool(&self) -> bool` | Bool value | `Expr` (bool lit) or `Lit` (bool) |

## Built-in `Attribute` Impls

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
| `Option<T: Attribute>` | `Some(T::from_arg(&flag_arg)?)` | `Some(T::from_arg(arg)?)` | `Some(T::from_arg(arg)?)` | `None` |
| `Vec<T: Attribute>` | error | error | each inner arg via `T::from_arg` | `vec![]` |
| `Args` | error | error | nested args directly | `Args::new()` |

## Files to Modify

| File | Change |
|---|---|
| `crates/core/src/meta/arg.rs` | Add `as_flag`, `as_str`, `as_int`, `as_float`, `as_char`, `as_bool` |
| `crates/core/src/meta/attribute.rs` | **New** — `Attribute` trait + all built-in impls |
| `crates/core/src/meta/mod.rs` | Add `mod attribute; pub use attribute::*;` |

## Tests

### Arg accessors
- `as_flag` on `Flag` variant → returns ident
- `as_str` on `Expr` with string lit → returns value
- `as_str` on `Lit` with string → returns value
- `as_int::<i64>` on `Lit` with int → returns value
- `as_flag` on non-Flag → panics
- `as_str` on `Flag` → panics

### `Attribute` impls
- `bool::from_arg` with `Flag` → `true`
- `bool` absent → `false`
- `String::from_arg` with string expr → value
- `i64::from_arg` with int lit → value
- `Option<String>::from_arg` present → `Some(value)`
- `Option<String>` absent → `None`
- `Vec<String>::from_arg` with `List` → collected values
- `Vec<String>` absent → `vec![]`
- `syn::Ident::from_arg` with `Flag` → the ident
- `syn::Expr::from_arg` with `Expr` → the expr
- Wrong type → `Err`
