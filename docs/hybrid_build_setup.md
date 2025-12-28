# Hybrid Build System Setup Guide

[TOC]

## Overview

This guide provides practical instructions for setting up and using the hybrid build system that supports both GN/Ninja and Cargo during the transition period. This allows teams to gradually migrate to Cargo while maintaining the ability to build with the existing GN/Ninja system.

## Prerequisites

- Existing Chromium/Crustonium checkout
- Rust toolchain installed (via `//third_party/rust-toolchain` or rustup)
- GN and Ninja already working
- CMake 3.15+ (for C++ integration in Cargo builds)

## Quick Start

### For Component Owners: Adding Cargo Support to Your Component

**Step 1: Create Cargo.toml**

```bash
cd your/component/
```

Create `Cargo.toml`:
```toml
[package]
name = "chromium-your-component"
version = "1.0.0"
edition = "2021"
license = "BSD-3-Clause"

[dependencies]
# Your dependencies here
cxx = "1.0"
log = "0.4"

[build-dependencies]
cxx-build = "1.0"

[lib]
crate-type = ["staticlib", "rlib"]
name = "your_component"
path = "src/lib.rs"
```

**Step 2: Keep BUILD.gn Working**

Your existing `BUILD.gn` should continue to work:
```python
# BUILD.gn
import("//build/rust/rust_static_library.gni")

rust_static_library("your_component") {
  crate_root = "src/lib.rs"
  sources = [
    "src/lib.rs",
    # ... other sources
  ]
  
  # This enables Cargo build as fallback
  enable_cargo_build = true  # NEW FLAG
  cargo_toml = "Cargo.toml"  # NEW FLAG
}
```

**Step 3: Test Both Build Systems**

```bash
# Test GN build (existing)
autoninja -C out/Default your/component:your_component

# Test Cargo build (new)
cd your/component
cargo build --lib
```

### For Developers: Using the Hybrid System

**Choose your build system**:

```bash
# Option 1: Build everything with GN/Ninja (current default)
export BUILD_SYSTEM=gn
./build.sh

# Option 2: Build Rust with Cargo, C++ with GN/Ninja
export BUILD_SYSTEM=hybrid
./build.sh

# Option 3: Build everything with Cargo (future)
export BUILD_SYSTEM=cargo
./build.sh
```

## Workspace Setup

### Root Cargo Workspace

Create `Cargo.toml` at repository root:

```toml
[workspace]
resolver = "2"

# All first-party Rust crates
members = [
    "components/qr_code_generator",
    "components/user_data_importer/utility/parsing_ffi",
    "components/facilitated_payments/core/validation",
    "device/bluetooth/bluez/ble_scan_parser",
    "media/filters",
    "testing/rust_gtest_interop",
    # Add new components here as they get Cargo.toml files
]

# Exclude generated code and third-party
exclude = [
    "third_party",
    "out",
    "target",
]

[workspace.dependencies]
# Common dependencies with unified versions
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
log = "0.4"
cxx = "1.0"

[workspace.package]
edition = "2021"
license = "BSD-3-Clause"
repository = "https://github.com/rexlunae/crustonium"

# Optimized profiles
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 16

[profile.dev]
opt-level = 0

# Production profile (even more optimized)
[profile.production]
inherits = "release"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

### Cargo Configuration

Create `.cargo/config.toml`:

```toml
[build]
# Use all CPU cores
jobs = 0

# Incremental compilation
incremental = true

# Use vendored crates
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "third_party/rust"

# Platform-specific linker configuration
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-msvc]
linker = "lld-link.exe"

[target.x86_64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Aliases for convenience
[alias]
br = "build --release"
tr = "test --release"
b = "build --workspace"
t = "test --workspace"
c = "check --workspace"
```

## Component Migration Templates

### Pure Rust Component

**Directory structure**:
```
your_component/
├── Cargo.toml
├── BUILD.gn              # Kept for backward compatibility
├── src/
│   ├── lib.rs
│   ├── module.rs
│   └── tests.rs
└── README.md
```

**Cargo.toml**:
```toml
[package]
name = "chromium-your-component"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
# Use workspace dependencies where possible
serde = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
# Test-only dependencies

[lib]
crate-type = ["rlib"]
```

**BUILD.gn**:
```python
import("//build/rust/rust_static_library.gni")

rust_static_library("your_component") {
  crate_root = "src/lib.rs"
  sources = [
    "src/lib.rs",
    "src/module.rs",
  ]
  
  # Enable Cargo build
  enable_cargo_build = true
  cargo_toml = "Cargo.toml"
  
  deps = [
    # GN dependencies
  ]
}
```

### Rust with C++ FFI

**Directory structure**:
```
your_component/
├── Cargo.toml
├── BUILD.gn
├── src/
│   ├── lib.rs
│   ├── ffi.rs            # cxx bridge
│   ├── implementation.rs
│   └── tests.rs
├── cpp/
│   ├── wrapper.cc        # C++ side of FFI
│   └── wrapper.h
└── build.rs              # Build script
```

**Cargo.toml**:
```toml
[package]
name = "chromium-your-component"
version.workspace = true
edition.workspace = true

[dependencies]
cxx = { workspace = true }

[build-dependencies]
cxx-build = "1.0"

[lib]
crate-type = ["staticlib", "rlib"]
```

**build.rs**:
```rust
fn main() {
    // cxx bridge
    cxx_build::bridge("src/ffi.rs")
        .file("cpp/wrapper.cc")
        .flag_if_supported("-std=c++17")
        .compile("your_component_cxx");
    
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=cpp/wrapper.cc");
    println!("cargo:rerun-if-changed=cpp/wrapper.h");
}
```

**src/ffi.rs**:
```rust
#[cxx::bridge(namespace = "chromium::your_component")]
mod ffi {
    extern "Rust" {
        type YourComponent;
        
        fn create() -> Box<YourComponent>;
        fn process(self: &YourComponent, data: &[u8]) -> Result<Vec<u8>>;
    }
}

pub struct YourComponent {
    // implementation
}

fn create() -> Box<YourComponent> {
    Box::new(YourComponent::new())
}

fn process(component: &YourComponent, data: &[u8]) -> Result<Vec<u8>, String> {
    component.process_impl(data)
        .map_err(|e| e.to_string())
}
```

### Component with C++ Dependencies

**Cargo.toml**:
```toml
[package]
name = "chromium-component-with-cpp"
version.workspace = true
edition.workspace = true

[dependencies]
cxx = { workspace = true }

[build-dependencies]
cc = "1.0"
cxx-build = "1.0"

[lib]
crate-type = ["staticlib", "rlib"]
```

**build.rs**:
```rust
fn main() {
    // Build C++ dependencies
    cc::Build::new()
        .cpp(true)
        .file("cpp/dependency.cc")
        .file("cpp/utilities.cc")
        .flag("-std=c++17")
        .warnings(false)  // Suppress warnings from legacy code
        .compile("cpp_dependencies");
    
    // cxx bridge
    cxx_build::bridge("src/ffi.rs")
        .file("cpp/wrapper.cc")
        .flag_if_supported("-std=c++17")
        .include("cpp")  // Add include path
        .compile("component_cxx");
    
    // Link against C++ stdlib
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "linux" | "macos" => println!("cargo:rustc-link-lib=stdc++"),
        "windows" => {}, // MSVC links automatically
        _ => {}
    }
}
```

## Build Scripts

### Master Build Script

Create `build.sh` at repository root:

```bash
#!/bin/bash
# build.sh - Unified build script for hybrid system

set -e

BUILD_SYSTEM="${BUILD_SYSTEM:-gn}"
BUILD_CONFIG="${BUILD_CONFIG:-Default}"
CARGO_PROFILE="${CARGO_PROFILE:-dev}"

echo "=== Chromium/Crustonium Build ==="
echo "Build System: $BUILD_SYSTEM"
echo "Configuration: $BUILD_CONFIG"
echo ""

case "$BUILD_SYSTEM" in
    gn)
        echo "Building with GN/Ninja..."
        if [ ! -d "out/$BUILD_CONFIG" ]; then
            gn gen "out/$BUILD_CONFIG"
        fi
        ninja -C "out/$BUILD_CONFIG" chrome
        ;;
        
    cargo)
        echo "Building with Cargo..."
        
        # Build Rust workspace
        if [ "$CARGO_PROFILE" = "release" ]; then
            cargo build --workspace --release
        else
            cargo build --workspace
        fi
        
        # Note: Pure Cargo build assumes all code is Rust
        # For hybrid period, use 'hybrid' mode
        ;;
        
    hybrid)
        echo "Building with hybrid system..."
        
        # Step 1: Build Rust components with Cargo
        echo "Step 1/2: Building Rust components with Cargo..."
        if [ "$CARGO_PROFILE" = "release" ]; then
            cargo build --workspace --release
        else
            cargo build --workspace
        fi
        
        # Step 2: Build C++ and link with Rust components
        echo "Step 2/2: Building C++ components with GN/Ninja..."
        if [ ! -d "out/$BUILD_CONFIG" ]; then
            gn gen "out/$BUILD_CONFIG" --args="use_cargo_rust=true"
        fi
        ninja -C "out/$BUILD_CONFIG" chrome
        
        echo "Hybrid build complete!"
        ;;
        
    *)
        echo "Error: Unknown build system '$BUILD_SYSTEM'"
        echo "Valid options: gn, cargo, hybrid"
        exit 1
        ;;
esac

echo ""
echo "Build complete!"
```

Make it executable:
```bash
chmod +x build.sh
```

### Test Script

Create `test.sh`:

```bash
#!/bin/bash
# test.sh - Unified test script

set -e

BUILD_SYSTEM="${BUILD_SYSTEM:-gn}"
TEST_FILTER="${TEST_FILTER:-}"

echo "=== Running Tests ==="
echo "Build System: $BUILD_SYSTEM"
echo ""

case "$BUILD_SYSTEM" in
    gn)
        echo "Running tests via GN/Ninja..."
        ninja -C out/Default unit_tests
        ./out/Default/unit_tests $TEST_FILTER
        ;;
        
    cargo)
        echo "Running tests via Cargo..."
        cargo test --workspace $TEST_FILTER
        ;;
        
    hybrid)
        echo "Running Rust tests via Cargo..."
        cargo test --workspace $TEST_FILTER
        
        echo ""
        echo "Running C++ tests via GN/Ninja..."
        ninja -C out/Default unit_tests
        ./out/Default/unit_tests $TEST_FILTER
        ;;
        
    *)
        echo "Error: Unknown build system '$BUILD_SYSTEM'"
        exit 1
        ;;
esac

echo ""
echo "Tests complete!"
```

Make it executable:
```bash
chmod +x test.sh
```

## CI/CD Integration

### GitHub Actions Workflow

Create `.github/workflows/hybrid-build.yml`:

```yaml
name: Hybrid Build CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  # Cargo-only build (for Rust components)
  cargo-build:
    name: Cargo Build
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache Cargo
      uses: Swatinem/rust-cache@v2
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Clippy
      run: cargo clippy --workspace --all-targets -- -D warnings
    
    - name: Build
      run: cargo build --workspace --verbose
    
    - name: Test
      run: cargo test --workspace --verbose
  
  # Hybrid build (Cargo + GN/Ninja)
  hybrid-build:
    name: Hybrid Build
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
      with:
        fetch-depth: 0  # Need full history for GN
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y ninja-build python3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Cache Cargo
      uses: Swatinem/rust-cache@v2
    
    - name: Hybrid build
      env:
        BUILD_SYSTEM: hybrid
      run: ./build.sh
    
    - name: Run tests
      env:
        BUILD_SYSTEM: hybrid
      run: ./test.sh
```

## Troubleshooting

### Common Issues

**Issue 1: Cargo can't find vendored crates**

```bash
# Solution: Ensure .cargo/config.toml points to correct directory
cat .cargo/config.toml
# Should have:
# [source.vendored-sources]
# directory = "third_party/rust"

# Verify vendored crates exist
ls third_party/rust/
```

**Issue 2: Linker errors with C++**

```rust
// build.rs - Ensure C++ stdlib is linked
let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
match target_os.as_str() {
    "linux" | "macos" => {
        println!("cargo:rustc-link-lib=stdc++");
    },
    "windows" => {
        // MSVC automatically links C++ stdlib
    },
    _ => {}
}
```

**Issue 3: GN can't find Cargo-built libraries**

```python
# BUILD.gn - Add Cargo output to search path
rust_static_library("component") {
  enable_cargo_build = true
  cargo_toml = "Cargo.toml"
  
  # Tell GN where to find Cargo output
  cargo_output_dir = "$root_build_dir/cargo_output"
}
```

**Issue 4: Incremental builds not working**

```bash
# Cargo incremental builds
export CARGO_INCREMENTAL=1

# GN incremental builds
gn gen out/Default --args='is_component_build=true'
```

**Issue 5: Build scripts fail on Windows**

```rust
// build.rs - Use std::env for paths, not hardcoded /
use std::path::PathBuf;

let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
let source = PathBuf::from("cpp/source.cc");
```

### Performance Tuning

**Parallel builds**:
```bash
# Cargo
export CARGO_BUILD_JOBS=8

# Ninja
ninja -C out/Default -j 8
```

**Build caching**:
```bash
# Install sccache
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=$HOME/.cache/sccache

# Check stats
sccache --show-stats
```

**Optimize dependencies**:
```toml
# Cargo.toml - Reduce build time
[profile.dev.package."*"]
opt-level = 0      # Don't optimize dependencies in dev

[profile.release]
lto = "thin"       # Thin LTO is faster than fat
codegen-units = 16 # More units = faster compile, slightly slower runtime
```

## Migration Checklist

When migrating a component to Cargo:

- [ ] Create `Cargo.toml` in component directory
- [ ] Update root `Cargo.toml` to include new component in workspace
- [ ] Keep `BUILD.gn` working with `enable_cargo_build = true`
- [ ] Create `build.rs` if C++ integration needed
- [ ] Test cargo build: `cargo build -p component-name`
- [ ] Test GN build: `ninja -C out/Default component:target`
- [ ] Test hybrid build: `BUILD_SYSTEM=hybrid ./build.sh`
- [ ] Update component README with build instructions
- [ ] Update CI/CD if needed
- [ ] Get code review from build system team

## Best Practices

1. **Keep BUILD.gn and Cargo.toml in sync**: When adding dependencies, update both
2. **Use workspace dependencies**: Avoid version conflicts
3. **Vendor dependencies**: Don't rely on network access during build
4. **Test both build systems**: Ensure neither breaks
5. **Document build scripts**: Explain complex `build.rs` logic
6. **Cache aggressively**: Use sccache and cargo cache
7. **Monitor build times**: Ensure no regressions
8. **Update incrementally**: Don't migrate everything at once

## Getting Help

- **Documentation**: See `/docs/cargo_adoption_plan.md`
- **Issues**: File issues with `build-system` label
- **Slack**: `#cargo-migration` channel
- **Mailing list**: `build-dev@chromium.org`
- **Office hours**: Wednesdays 2-3 PM (check calendar)

## Next Steps

1. Read the [Cargo Adoption Plan](cargo_adoption_plan.md)
2. Join the Cargo migration working group
3. Identify components to migrate
4. Follow this guide to set up hybrid builds
5. Submit your first migration CL

---

**Happy building!** 🦀
