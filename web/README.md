# web

Leptos + Stylance wasm frontend for `websh`, consuming the local `shell` crate.

## Requirements

- Rust stable
- `wasm32-unknown-unknown` target
- `trunk`
- `stylance-cli`

## Quick start

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install stylance-cli
cd web
trunk serve
```

The Trunk pre-build hook runs Stylance and writes bundled CSS to `styles/index.css`.

## Structure

- `src/components/*`: reusable atomic UI components (one folder per component, lowercase)
- `src/pages/*`: page architecture/composition
- `styles/index.css`: Stylance output file consumed by Trunk
