# Flagfy Build Engiine

WebAssembly compiler for [Flagfy](https://github.com/flagfy) decisioning specs, written in Rust.

Takes YAML project definitions — contexts, segments and decisions — and compiles them into versioned, environment-aware bundles ready to be published and evaluated at runtime.

> If you're looking for the JS/Node.js package, see [@flagfy/build-engine on npm](https://www.npmjs.com/package/@flagfy/build-engine).

## Installation

```toml
[dependencies]
flagfy-build-engine = "0.1"
```

## Core concepts

- **Contexts** define the shape of evaluation inputs (e.g. `user.country`, `user.orders.count`)
- **Segments** define named groups with match rules against contexts
- **Decisions** define evaluation trees that resolve to outcomes based on segments
- **Environments** scope any of the above to specific deploy targets (`dev`, `prod`, `stag`, etc.)
- **Bundles** are the compiled, content-hashed output per environment

## Usage

```rust
use flagfy_compiler::{BuildInput, FileEntry, build};

let input = BuildInput {
    project: std::fs::read_to_string("project.yaml").unwrap(),
    globals: std::fs::read_to_string("globals.yaml").unwrap(),
    contexts: vec![
        FileEntry {
            name: "user".into(),
            content: std::fs::read_to_string(".flagfy/contexts/user.yaml").unwrap(),
        },
    ],
    segments: vec![
        FileEntry {
            name: "checkout".into(),
            content: std::fs::read_to_string(".flagfy/segments/checkout.yaml").unwrap(),
        },
    ],
    decisions: vec![
        FileEntry {
            name: "checkout".into(),
            content: std::fs::read_to_string(".flagfy/decisions/checkout.yaml").unwrap(),
        },
    ],
};

let output = build(input)?;

// output.files is a HashMap<String, serde_json::Value>
// keys are relative paths: "meta.json", "bundles/prod.json", "quick_checks/prod.json", etc.
for (path, content) in &output.files {
    println!("{}: {}", path, content);
}
```

## Environment filtering

By default every file is included in all environments. To restrict a file, add a `meta` block:

```yaml
meta:
  environments: [prod, stag]

name: checkout_prod
decisions:
  gateway:
    evaluate:
      - when:
          segment: checkout.high_value
        then: fast_gateway
      - when:
          default: true
        then: global_gateway
```

Omit `meta` or use `environments: all` to include a file everywhere. Environments are discovered automatically from `project.yaml` and from every `meta.environments` reference across all files — no central registry needed.

## Output

`build()` returns a `BuildOutput` with a `files` map:

| path | description |
|---|---|
| `meta.json` | `spec`, `generated_at`, `globals`, `active_environments` |
| `bundles/all.json` | bundle included in every environment |
| `bundles/<env>.json` | filtered bundle for `<env>` |
| `quick_checks/all.json` | `checksum`, `bundle_uri`, `env_version` |
| `quick_checks/<env>.json` | same + `publish_path`, `bootstrap_pointer` |

Each bundle has its own `core_version` — a SHA-256 hash of its filtered contents — so hashes differ across environments when content differs.

## API

### `build(input: BuildInput) -> Result<BuildOutput, CompilerError>`

Main entry point. Parses all YAML inputs, discovers environments, compiles per-environment bundles and returns the file map.

### `BuildInput`

| field | type | description |
|---|---|---|
| `project` | `String` | Raw YAML content of `project.yaml` |
| `globals` | `String` | Raw YAML content of `globals.yaml` |
| `contexts` | `Vec<FileEntry>` | Context schema files |
| `segments` | `Vec<FileEntry>` | Segment definition files |
| `decisions` | `Vec<FileEntry>` | Decision tree files |

### `FileEntry`

```rust
pub struct FileEntry {
    pub name: String,     // filename without extension
    pub content: String,  // raw YAML content
}
```

### `BuildOutput`

```rust
pub struct BuildOutput {
    pub spec: String,
    pub generated_at: String,
    pub globals: HashMap<String, serde_json::Value>,
    pub active_environments: Vec<String>,
    pub files: HashMap<String, serde_json::Value>,
}
```

## WASM

This crate is also compiled to WebAssembly and published as [@flagfy/build-engine](https://www.npmjs.com/package/@flagfy/build-engine) on npm. The `build_state` WASM export wraps `build()` with JSON serialization on both ends.

To build the WASM target yourself:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/flagfy_build_engine.wasm \
  --out-dir pkg --target nodejs
```

## License

MIT