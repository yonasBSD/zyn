# Errors and Warnings

The four diagnostic directives cover all compiler output needs: `@throw` (error), `@warn` (warning), `@note` (informational), `@help` (suggestion).

## `@throw` — Compile Error

Emits a hard compile error that halts the build:

```rust,zyn
zyn! {
    @if (!valid) {
        @throw "expected a struct"
    }
}
```

```bash
error: expected a struct
  --> src/lib.rs:12:9
   |
12 |         @throw "expected a struct"
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## `@warn` — Compiler Warning

Emits a non-fatal warning. Compilation continues normally:

```rust,zyn
zyn! {
    @if (deprecated) {
        @warn "this usage is deprecated, use `new_api` instead"
    }
}
```

```bash
warning: this usage is deprecated, use `new_api` instead
  --> src/lib.rs:8:9
   |
 8 |         @warn "this usage is deprecated, use `new_api` instead"
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## `@note` — Informational Note

Emits a standalone informational note diagnostic:

```rust,zyn
zyn! {
    @note "this field is deprecated since v2"
}
```

## `@help` — Help Suggestion

Emits a standalone suggestion:

```rust,zyn
zyn! {
    @help "consider using `Builder::new()` instead"
}
```

## Nesting — Attach Notes and Help to Errors/Warnings

`@throw` and `@warn` accept an optional body block containing `@note` and `@help` children. These are attached to the parent diagnostic:

```rust,zyn
zyn! {
    @if (fields.is_empty()) {
        @throw "struct must have at least one field" {
            @note "zero-field structs cannot derive this trait"
            @help "add at least one public field"
        }
    }
}
```

```bash
error: struct must have at least one field
note: zero-field structs cannot derive this trait
help: add at least one public field
  --> src/lib.rs:8:9
   |
 8 |         @throw "struct must have at least one field" {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```rust,zyn
zyn! {
    @if (is_legacy) {
        @warn "using deprecated API" {
            @note "will be removed in v3"
            @help "migrate to `new_api()`"
        }
    }
}
```

`@warn` does not halt compilation. `@throw` is a hard error.
