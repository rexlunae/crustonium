# Workspace Management Best Practices

**Phase 1.3: Documentation and Training**

Guidelines for managing Cargo workspaces in the Chromium monorepo.

[TOC]

## Overview

A Cargo workspace is a collection of packages that share common dependencies, build configuration, and output directory. This guide provides best practices for maintaining the Chromium Cargo workspace.

## Workspace Structure

### Recommended Layout

```
chromium/
├── Cargo.toml                  # Workspace root manifest
├── Cargo.lock                  # Dependency lock file (commit this!)
├── .cargo/
│   └── config.toml            # Workspace configuration
├── crates/                     # First-party Rust code
│   ├── base/                  # Core utilities
│   ├── component_a/
│   └── component_b/
├── prototypes/                 # Experimental code
│   └── research/
└── target/                     # Build output (DO NOT commit)
```

### Directory Organization

**Principle**: Group by functionality, not by language

**Good**:
```
components/
├── qr_code_generator/
│   ├── BUILD.gn
│   ├── Cargo.toml
│   └── src/
```

**Avoid**:
```
rust/
├── qr_code_generator/
cpp/
├── qr_code_generator/
```

## Workspace Manifest (Root Cargo.toml)

### Template

```toml
[workspace]
# Use resolver 2 for better dependency resolution
resolver = "2"

# List all workspace members
members = [
    "crates/base",
    "crates/component_a",
    "crates/component_b",
    "prototypes/*",
]

# Exclude build artifacts and third-party code
exclude = [
    "third_party",
    "out",
    "target",
]

# Shared dependency versions (critical for consistency)
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
log = "0.4"
cxx = "1.0"

# Shared package metadata
[workspace.package]
version = "1.0.0"
edition = "2021"
license = "BSD-3-Clause"
repository = "https://github.com/rexlunae/crustonium"

# Workspace-wide build profiles
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 16

[profile.production]
inherits = "release"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

### Best Practices for Root Manifest

**1. Use workspace.dependencies for all shared dependencies**
```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
```

Benefits:
- Single source of truth for versions
- Prevents version conflicts
- Easier to update
- Better deduplication

**2. Group related dependencies**
```toml
[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# FFI
cxx = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Testing (dev-dependencies)
criterion = "0.5"
```

**3. Use glob patterns sparingly**
```toml
members = [
    "crates/*",           # OK: All direct children
    "prototypes/*",       # OK: Experiments
    # "crates/**",        # AVOID: Recursive, hard to track
]
```

**4. Document exclusions**
```toml
exclude = [
    "third_party",        # External dependencies
    "out",                # GN/Ninja output
    "target",             # Cargo output
    "deprecated",         # Old code
]
```

## Member Package Configuration

### Template

```toml
[package]
name = "chromium-component-name"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
# Use workspace dependencies
serde = { workspace = true }
log = { workspace = true }

# Local workspace members
chromium-base = { path = "../base" }

# Optional external dependencies
reqwest = { version = "0.11", optional = true }

[dev-dependencies]
criterion = { workspace = true }

[features]
default = ["std"]
std = []
networking = ["dep:reqwest"]
```

### Naming Conventions

**Package Names**:
- Prefix with `chromium-` to avoid conflicts
- Use kebab-case: `chromium-qr-code-generator`
- Match component path when possible

**Library Names**:
- Use snake_case: `qr_code_generator`
- Can differ from package name
- Defined in `[lib]` section

```toml
[package]
name = "chromium-qr-code-generator"  # Package name

[lib]
name = "qr_code_generator"           # Library name (snake_case)
```

## Dependency Management

### Version Management

**Rule 1: Use workspace dependencies for shared deps**

```toml
# In root Cargo.toml
[workspace.dependencies]
serde = "1.0"

# In member Cargo.toml
[dependencies]
serde = { workspace = true }
```

**Rule 2: Pin versions of critical dependencies**

```toml
[workspace.dependencies]
# Critical security dependency - pin exact version
ring = "=0.17.7"

# General dependency - use compatible range
serde = "1.0"
```

**Rule 3: Document version constraints**

```toml
[workspace.dependencies]
# Using 1.0 because feature X requires it
serde = { version = "1.0", features = ["derive"] }

# Pinned due to CVE-2024-XXXXX
old_lib = "=0.5.2"
```

### Handling Path Dependencies

**For workspace members**:
```toml
[dependencies]
chromium-base = { path = "../base" }
```

**For local development of external crates**:
```toml
[patch.crates-io]
my_crate = { path = "../local-fork/my_crate" }
```

### Optional Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", optional = true }

[features]
networking = ["dep:reqwest"]
```

Usage:
```rust
#[cfg(feature = "networking")]
use reqwest;
```

## Features and Conditional Compilation

### Feature Design

**Keep features orthogonal**:

**Good**:
```toml
[features]
default = ["std"]
std = []
alloc = []
networking = ["std", "dep:reqwest"]
```

**Bad**:
```toml
[features]
everything = ["std", "alloc", "networking"]  # Too coarse
```

### Feature Naming

- Use `std` for standard library
- Use `no_std` only if default is `std`
- Use descriptive names: `networking`, `compression`
- Avoid negative features when possible

### Testing Features

```bash
# Test default features
cargo test -p chromium-component

# Test no default features
cargo test -p chromium-component --no-default-features

# Test all features
cargo test -p chromium-component --all-features

# Test specific feature combination
cargo test -p chromium-component --features "networking,compression"
```

## Build Configuration

### .cargo/config.toml

```toml
[build]
# Incremental compilation (faster rebuilds)
incremental = true

# Target directory
target-dir = "target"

# Platform-specific linker configuration
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-msvc]
linker = "lld-link.exe"

[target.x86_64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Vendored dependencies
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "third_party/rust"

# Aliases
[alias]
b = "build --workspace"
t = "test --workspace"
c = "check --workspace"
```

### When to Vendor Dependencies

**Vendor when**:
- Production builds
- Reproducible builds required
- Offline builds needed
- Corporate policy requires it

**Don't vendor for**:
- Development/prototyping
- Rapid iteration
- CI with good network

```bash
# Vendor dependencies
cargo vendor third_party/rust

# Update .cargo/config.toml to use vendored sources
```

## Version Control

### What to Commit

**Always commit**:
- ✅ `Cargo.toml` (all of them)
- ✅ `Cargo.lock` (workspace root)
- ✅ `.cargo/config.toml`
- ✅ `src/` directories
- ✅ `build.rs` files

**Never commit**:
- ❌ `target/` directory
- ❌ `Cargo.lock` in libraries (workspace members)
- ❌ `*.orig` files
- ❌ Editor-specific files

### .gitignore

```gitignore
# Cargo build output
/target/

# Cargo lock files (except workspace root)
# (Include /Cargo.lock explicitly)
**/Cargo.lock
!/Cargo.lock

# Backup files
**/*.rs.bk
**/*.orig

# IDE files
.idea/
.vscode/
*.swp
```

## Performance Optimization

### Build Performance

**1. Use sccache**
```bash
# Install
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache

# Check stats
sccache --show-stats
```

**2. Optimize dependencies**
```toml
# Reduce build time in dev
[profile.dev.package."*"]
opt-level = 0
```

**3. Use cargo-nextest**
```bash
# Faster test runner
cargo install cargo-nextest
cargo nextest run
```

**4. Parallel compilation**
```bash
# Use all cores (default)
cargo build

# Limit to N jobs
cargo build -j4
```

### Incremental Compilation

```toml
# .cargo/config.toml
[build]
incremental = true  # Enabled by default in dev

# Or via environment
export CARGO_INCREMENTAL=1
```

### Dependency Optimization

**Minimize dependencies**:
```bash
# Show dependency tree
cargo tree

# Find duplicate dependencies
cargo tree --duplicates

# Find unused dependencies
cargo install cargo-udeps
cargo udeps
```

## Workspace Hygiene

### Regular Maintenance

**Monthly**:
```bash
# Update dependencies
cargo update

# Check for security advisories
cargo audit

# Check licenses
cargo deny check licenses

# Run clippy
cargo clippy --workspace -- -D warnings
```

**Before Release**:
```bash
# Full clean build
cargo clean
cargo build --release --workspace

# All tests
cargo test --workspace --all-features

# Check documentation
cargo doc --workspace --no-deps
```

### Dependency Auditing

Create `deny.toml`:
```toml
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
]

[bans]
multiple-versions = "warn"
wildcards = "deny"

[advisories]
vulnerability = "deny"
unmaintained = "warn"
```

Run checks:
```bash
cargo deny check
```

## Common Pitfalls

### 1. Circular Dependencies

**Problem**: Crate A depends on B, B depends on A

**Solution**: Extract common code to new crate C, both depend on C

### 2. Version Conflicts

**Problem**: Different crates use different versions of same dependency

**Solution**: Use `[workspace.dependencies]`

### 3. Feature Flag Hell

**Problem**: Too many interdependent features

**Solution**: Keep features simple and orthogonal

### 4. Large Binary Size

**Problem**: Release binary is huge

**Solution**:
```toml
[profile.release]
strip = true          # Remove debug symbols
lto = "fat"           # Link-time optimization
codegen-units = 1     # Better optimization
opt-level = "z"       # Optimize for size
```

### 5. Slow Builds

**Problem**: Clean builds take forever

**Solutions**:
- Use sccache
- Reduce dependencies
- Use `cargo check` for development
- Optimize dependency features

## Workspace Commands Reference

```bash
# Build all workspace members
cargo build --workspace

# Test all workspace members
cargo test --workspace

# Check all workspace members (fast)
cargo check --workspace

# Format all workspace code
cargo fmt --all

# Lint all workspace code
cargo clippy --workspace

# Build specific package
cargo build -p chromium-component

# Update all dependencies
cargo update --workspace

# Show workspace structure
cargo metadata --format-version 1 | jq '.workspace_members'

# Clean all build artifacts
cargo clean
```

## Troubleshooting

### "Multiple packages with same name"

**Cause**: Duplicate package names in workspace

**Solution**: Rename one package

### "Failed to resolve patches"

**Cause**: Patch doesn't match dependency version

**Solution**: Update patch version or dependency version

### "Dependency not found"

**Cause**: Missing from workspace.dependencies

**Solution**: Add to root Cargo.toml `[workspace.dependencies]`

### "Incremental build failing"

**Cause**: Corrupted incremental state

**Solution**:
```bash
rm -rf target/debug/incremental
cargo clean -p chromium-component
cargo build
```

## Resources

- [Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo Best Practices](https://doc.rust-lang.org/cargo/guide/cargo-best-practices.html)
- [Chromium Cargo Adoption Plan](../../cargo_adoption_plan.md)

---

**Questions?** Contact `#cargo-migration` on Slack
