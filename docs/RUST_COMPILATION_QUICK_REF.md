# Rust Compilation Quick Reference

## Common Commands

### Building

```bash
# Build all Rust components
cargo build --workspace

# Build in release mode
cargo build --workspace --release

# Build specific component
cargo build -p qr-code-generator-ffi

# Clean build artifacts
cargo clean
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cargo test -p qr-code-generator-ffi

# Run tests with output
cargo test --workspace -- --nocapture
```

### Code Quality

```bash
# Run clippy linter
cargo clippy --workspace

# Fix clippy warnings
cargo clippy --workspace --fix

# Format code
cargo fmt --all

# Check formatting without changing files
cargo fmt --all -- --check
```

### Documentation

```bash
# Build documentation
cargo doc --workspace --no-deps

# Build and open documentation
cargo doc --workspace --no-deps --open
```

## Component Structure

```
component_name/
├── Cargo.toml           # Package configuration
├── build.rs             # Build script (for cxx)
├── src/ or .rs file     # Rust source code
└── tests/               # Integration tests (optional)
```

## Minimal Cargo.toml Template

```toml
[package]
name = "component-name"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "component_name"
crate-type = ["staticlib", "rlib"]

[dependencies]
cxx = { workspace = true }

[build-dependencies]
cxx-build = { workspace = true }
```

## Minimal build.rs Template

```rust
fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = manifest_dir
        .rsplit_once("/your/path")
        .unwrap()
        .0;

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .include(workspace_root)
        .compile("component_name");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
```

## Troubleshooting

### Error: Missing header file

**Solution**: Add workspace root to include path in `build.rs`:
```rust
.include(workspace_root)
```

### Error: Unresolved import

**Solution**: Add dependency to both workspace and component `Cargo.toml`

### Error: cxx bridge compilation fails

**Solution**: Check that all C++ headers in `extern "C++"` blocks exist

## Getting Help

- Check [RUST_COMPILATION.md](RUST_COMPILATION.md) for detailed guide
- Run `cargo help <command>` for command help
- Use `cargo --explain <error_code>` for error explanations
