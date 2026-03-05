# Backlog

Planned features for zyn, broken into sequential phases. Each phase builds on the previous.

## Phases

| Phase | Name | Status | Description |
|---|---|---|---|
| [1](./01-attribute-trait.md) | Attribute Trait | planned | `Attribute` trait + built-in scalar/container/syn impls in `zyn-core`. Typed `Arg` accessors. |
| [2](./02-derive-structs.md) | Derive Structs | planned | `#[derive(Attribute)]` for structs (attribute mode + argument mode) |
| [3](./03-derive-enums.md) | Derive Enums | planned | `#[derive(Attribute)]` for enums (discriminated union argument types) |
| [4](./04-element-integration.md) | Element Integration | planned | `#[zyn(from_input)]` on `#[element]` params — automatic `Attribute::attribute()` injection |
| [5](./05-error-accumulation.md) | Error Accumulation | planned | Collect and emit all validation errors together instead of short-circuiting |

## What Gets Replaced

| Current module | Replaced by | Phase |
|---|---|---|
| Manual `Arg`/`Args` querying | `Attribute` trait + `#[derive(Attribute)]` | 1–2 |
| `AttrExt`/`AttrsExt` as primary API | `Attribute::attribute` (name baked into derive) | 2 |

`Arg`, `Args`, `AttrExt`, `AttrsExt` remain as low-level internals used by `Attribute`, but are no longer the primary user-facing API.
