# zyn — VSCode Extension

Syntax highlighting for [zyn](https://github.com/aacebo/zyn) template macros in Rust files.

## What it highlights

- `@if`, `@else`, `@for`, `@match`, `@throw` — control flow keywords
- `@element_name(...)` — element invocations
- `{{ expr }}` — interpolation delimiters
- `| pipe_name` — pipe operators and names
- `| pipe_name:"arg"` — pipe arguments

## Install from source

```sh
cd editors/vscode
npm install -g @vscode/vsce
vsce package
code --install-extension zyn-0.0.1.vsix
```

Or symlink for development:

```sh
ln -s "$(pwd)" ~/.vscode/extensions/zyn
```

Then reload VSCode (`Ctrl+Shift+P` → "Reload Window").
