# Cargo Basics for Chromium Developers

**Phase 1.3: Documentation and Training**

This guide introduces Cargo to Chromium developers familiar with GN/Ninja.

[TOC]

## Introduction

Cargo is Rust's build system and package manager. While GN/Ninja focuses on flexibility and cross-language builds, Cargo emphasizes convention, simplicity, and deep Rust ecosystem integration.

### Why Cargo?

**Benefits for Chromium Development**:
- **Standard tooling**: Leverage the entire Rust ecosystem
- **Better IDE support**: rust-analyzer works seamlessly  
- **Dependency management**: Automated version resolution
- **Built-in testing**: Integrated test framework
- **Documentation**: Auto-generated docs from code
- **Benchmarking**: Built-in benchmark support

### GN/Ninja vs Cargo: Quick Comparison

| Feature | GN/Ninja | Cargo |
|---------|----------|-------|
| **Configuration** | BUILD.gn files | Cargo.toml files |
| **Build command** | `ninja -C out/Default` | `cargo build` |
| **Test command** | `out/Default/unit_tests` | `cargo test` |
| **Clean** | `rm -rf out/` | `cargo clean` |
| **Dependencies** | Manual in DEPS | Automatic via Cargo.toml |
| **Incremental builds** | Yes | Yes |
| **Cross-compilation** | Via toolchain args | Via `--target` flag |

## Getting Started

### Installation

Cargo comes with Rust:

```bash
# Check if already installed
cargo --version

# If not, install Rust (includes Cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update if needed
rustup update stable
```

### Your First Cargo Project

```bash
# Create new library
cargo new --lib my_component

# Create new binary
cargo new my_tool

# Directory structure created:
# my_component/
# ├── Cargo.toml      # Package manifest
# └── src/
#     └── lib.rs      # Source code
```

## Cargo.toml: The Manifest

Think of `Cargo.toml` as the Cargo equivalent of `BUILD.gn`.

### Basic Structure

```toml
[package]
name = "chromium-my-component"
version = "1.0.0"
edition = "2021"
license = "BSD-3-Clause"

[dependencies]
serde = "1.0"                    # Simple version
log = { version = "0.4", features = ["std"] }  # With features

[dev-dependencies]
criterion = "0.5"                # Only for tests/benchmarks

[build-dependencies]
cc = "1.0"                       # Only for build.rs

[lib]
name = "my_component"
path = "src/lib.rs"
crate-type = ["rlib", "staticlib"]
```

### Dependency Specifications

```toml
# Exact version
serde = "=1.0.100"

# Compatible version (semantic versioning)
serde = "1.0"          # >= 1.0.0, < 2.0.0
serde = "1.0.100"      # >= 1.0.100, < 1.1.0

# Any version in range
serde = ">=1.0, <1.5"

# Wildcard
serde = "1.*"

# Git dependency
my_lib = { git = "https://github.com/org/repo", tag = "v1.0" }

# Path dependency (for local crates)
my_lib = { path = "../my_lib" }

# Workspace dependency
my_lib = { workspace = true }
```

## Common Cargo Commands

### Building

```bash
# Build in debug mode (default)
cargo build

# Build in release mode (optimized)
cargo build --release

# Build specific package in workspace
cargo build -p chromium-qr-code-generator

# Build all workspace members
cargo build --workspace

# Just check (no linking, faster)
cargo check

# Build with verbose output
cargo build --verbose
```

### Testing

```bash
# Run all tests
cargo test

# Run tests in specific package
cargo test -p my_component

# Run specific test
cargo test test_name

# Run tests with output shown
cargo test -- --nocapture

# Run doc tests
cargo test --doc

# Run only unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_test_name
```

### Running

```bash
# Run binary (if package has bin)
cargo run

# Run with arguments
cargo run -- --arg1 value1

# Run specific binary
cargo run --bin my_tool

# Run example
cargo run --example example_name
```

### Other Useful Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for security advisories
cargo audit

# Show dependency tree
cargo tree

# Benchmark
cargo bench
```

## Workspaces

Chromium uses a Cargo workspace for the monorepo structure.

### Workspace Structure

```
chromium/
├── Cargo.toml          # Workspace root
├── .cargo/
│   └── config.toml    # Configuration
└── crates/
    ├── component1/
    │   ├── Cargo.toml
    │   └── src/
    └── component2/
        ├── Cargo.toml
        └── src/
```

### Root Cargo.toml

```toml
[workspace]
resolver = "2"

members = [
    "crates/component1",
    "crates/component2",
]

# Shared dependencies
[workspace.dependencies]
serde = "1.0"
log = "0.4"

# Shared metadata
[workspace.package]
version = "1.0.0"
edition = "2021"
license = "BSD-3-Clause"
```

### Using Workspace Dependencies

In member crates:

```toml
[package]
name = "chromium-component1"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { workspace = true }
component2 = { path = "../component2" }
```

### Workspace Commands

```bash
# Build all members
cargo build --workspace

# Test all members
cargo test --workspace

# Check all members
cargo check --workspace

# Update all members
cargo update --workspace
```

## Build Scripts (build.rs)

For C++ integration or code generation, use `build.rs`:

```rust
// build.rs in package root
use std::env;

fn main() {
    // Compile C++ code
    cc::Build::new()
        .cpp(true)
        .file("cpp/legacy.cc")
        .compile("legacy");
    
    // Set environment variables
    println!("cargo:rustc-link-lib=stdc++");
    
    // Re-run if files change
    println!("cargo:rerun-if-changed=cpp/");
}
```

## Features and Conditional Compilation

### Defining Features

```toml
[features]
# Default features enabled
default = ["std"]

# Feature definitions
std = []
networking = ["dep:reqwest"]
experimental = []

[dependencies]
reqwest = { version = "0.11", optional = true }
```

### Using Features

```rust
// In code
#[cfg(feature = "networking")]
fn network_operation() {
    // Only compiled when networking feature enabled
}

// Always check if feature is enabled
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
```

### Building with Features

```bash
# Build with specific features
cargo build --features networking

# Build without default features
cargo build --no-default-features

# Build with all features
cargo build --all-features
```

## Profiles and Optimization

### Built-in Profiles

```toml
# In Cargo.toml

[profile.dev]
opt-level = 0      # No optimization
debug = true

[profile.release]
opt-level = 3      # Full optimization
debug = false
lto = "thin"       # Link-time optimization

# Custom profile (inherits from release)
[profile.production]
inherits = "release"
lto = "fat"
codegen-units = 1
```

### Using Profiles

```bash
# Build with dev profile (default)
cargo build

# Build with release profile
cargo build --release

# Build with custom profile
cargo build --profile production
```

## Cross-Compilation

```bash
# List available targets
rustup target list

# Install target
rustup target add x86_64-pc-windows-msvc

# Build for target
cargo build --target x86_64-pc-windows-msvc

# Multiple targets
cargo build --target x86_64-unknown-linux-gnu
cargo build --target aarch64-apple-darwin
```

## Cargo Configuration

### .cargo/config.toml

```toml
# Project-specific config in .cargo/config.toml

[build]
target-dir = "target"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[alias]
b = "build"
t = "test"
r = "run"
```

## Integration with GN/Ninja (Hybrid Mode)

During migration, both systems coexist:

### Using Hybrid Build Script

```bash
# Build with Cargo only
./tools/cargo_migration/hybrid_build.sh --system cargo

# Build with both (hybrid)
./tools/cargo_migration/hybrid_build.sh --system hybrid

# Traditional GN build
./tools/cargo_migration/hybrid_build.sh --system gn
```

### Migrating a Component

1. **Create Cargo.toml** (use gn_to_cargo.py):
   ```bash
   python3 tools/cargo_migration/gn_to_cargo.py BUILD.gn --target my_target
   ```

2. **Add to workspace**:
   Edit root `Cargo.toml`, add to `members = [...]`

3. **Test build**:
   ```bash
   cargo build -p chromium-my-component
   ```

4. **Keep BUILD.gn** (for hybrid mode):
   ```python
   rust_static_library("my_component") {
     enable_cargo_build = true
     cargo_toml = "Cargo.toml"
   }
   ```

## Best Practices

### 1. Use Workspace Dependencies

**Good**:
```toml
[dependencies]
serde = { workspace = true }
```

**Bad**:
```toml
[dependencies]
serde = "1.0"  # Version could drift
```

### 2. Pin Versions in Cargo.lock

- **Always commit** `Cargo.lock` for applications and tools
- Ensures reproducible builds
- Lock file managed automatically by Cargo

### 3. Use cargo-deny

```bash
# Install
cargo install cargo-deny

# Check licenses
cargo deny check licenses

# Check security advisories
cargo deny check advisories
```

### 4. Leverage cargo-clippy

```bash
# Lint code
cargo clippy -- -D warnings

# Fix automatically (be careful!)
cargo clippy --fix
```

### 5. Document with Doc Comments

```rust
/// Processes input data and returns result.
///
/// # Arguments
///
/// * `data` - Input data to process
///
/// # Examples
///
/// ```
/// let result = process(&[1, 2, 3]);
/// assert_eq!(result.len(), 3);
/// ```
pub fn process(data: &[u8]) -> Vec<u8> {
    // ...
}
```

Generate docs:
```bash
cargo doc --open
```

## Common Pitfalls for GN Users

### 1. Forgetting to Add Dependencies

**GN**: Implicit includes often work  
**Cargo**: Must explicitly declare all dependencies

**Solution**: Add to `[dependencies]` in Cargo.toml

### 2. Circular Dependencies

**GN**: Sometimes allows circular deps  
**Cargo**: Strictly forbidden

**Solution**: Refactor code to break cycles

### 3. Different Path Resolution

**GN**: Paths relative to BUILD.gn  
**Cargo**: Paths relative to Cargo.toml

**Solution**: Adjust include paths in build.rs

### 4. Feature Flags vs GN Args

**GN**: `gn args` for configuration  
**Cargo**: `--features` flag

**Solution**: Use Cargo features for conditional compilation

## Quick Reference Card

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build
cargo check                    # Fast check, no linking

# Test
cargo test                     # Run all tests
cargo test test_name           # Run specific test
cargo test -- --nocapture      # Show output

# Run
cargo run                      # Run binary
cargo run -- --args            # With arguments

# Quality
cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo audit                    # Security check

# Documentation
cargo doc --open               # Generate and view docs

# Maintenance
cargo clean                    # Clean build artifacts
cargo update                   # Update dependencies
cargo tree                     # Show dependency tree

# Workspace
cargo build --workspace        # Build all members
cargo test --workspace         # Test all members
```

## Next Steps

1. **Try it out**: Build a prototype with Cargo
2. **Read**: [The Cargo Book](https://doc.rust-lang.org/cargo/)
3. **Practice**: Convert a small component from GN
4. **Ask**: Slack `#cargo-migration` for help

## Additional Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/)
- [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html)
- [Chromium Rust Guide](../rust.md)
- [Migration Guide](migration_guide.md)
- [Quick Reference](quick_reference.md)

---

**Questions?** Contact `#cargo-migration` on Slack or `build-dev@chromium.org`
