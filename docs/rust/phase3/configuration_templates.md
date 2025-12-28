# Phase 3: Configuration Templates and Examples

This document provides ready-to-use configuration files, build scripts, and integration examples for Phase 3 component migrations.

[TOC]

## Configuration Files

### 1. IDE Configuration

#### VSCode Settings (`.vscode/settings.json`)

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.linkedProjects": [
    "Cargo.toml"
  ],
  "rust-analyzer.cargo.buildScripts.enable": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.check.workspace": true,
  "rust-analyzer.diagnostics.disabled": [
    "unresolved-proc-macro"
  ],
  "rust-analyzer.rustfmt.extraArgs": [
    "+nightly"
  ],
  "rust-analyzer.files.excludeDirs": [
    "target",
    "out",
    ".git"
  ],
  "rust-analyzer.imports.granularity.group": "crate",
  "rust-analyzer.imports.prefix": "crate",
  "files.watcherExclude": {
    "**/target/**": true,
    "**/out/**": true
  },
  "search.exclude": {
    "**/target": true,
    "**/out": true
  }
}
```

#### CLion/IntelliJ Settings

```xml
<!-- .idea/workspace.xml additions -->
<component name="CargoSettings">
  <option name="toolchainHomeDirectory" value="$USER_HOME$/.cargo" />
  <option name="autoUpdateEnabled" value="true" />
</component>

<component name="RustProjectSettings">
  <option name="compileAllTargets" value="false" />
  <option name="runExternalLinterOnTheFly" value="true" />
  <option name="toolchainHomeDirectory" value="$USER_HOME$/.rustup/toolchains/stable-x86_64-unknown-linux-gnu" />
  <option name="version" value="2" />
</component>
```

### 2. Clippy Configuration (`.clippy.toml`)

```toml
# Clippy lint configuration for Phase 3
msrv = "1.75.0"  # Minimum Supported Rust Version

# Allowed lints (project-specific exceptions)
allow = [
    "too_many_arguments",     # Complex FFI functions
    "type_complexity",         # FFI type aliases
    "module_name_repetitions", # Chromium naming conventions
]

# Warnings as errors for CI
warn = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::cargo",
]

# Explicitly denied lints (safety and correctness)
deny = [
    "clippy::unwrap_used",
    "clippy::expect_used",
    "clippy::panic",
    "clippy::unimplemented",
    "clippy::todo",
    "clippy::unreachable",
    "clippy::mem_forget",
    "clippy::cast_ptr_alignment",
]

# Nursery lints (experimental but useful)
nursery = [
    "clippy::redundant_pub_crate",
    "clippy::use_self",
]
```

### 3. Rustfmt Configuration (`rustfmt.toml`)

```toml
# Rust formatting configuration for Chromium
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
fn_call_width = 80
attr_fn_like_width = 70
struct_lit_width = 18
struct_variant_width = 35
array_width = 60
chain_width = 60
single_line_if_else_max_width = 50
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
normalize_comments = true
normalize_doc_attributes = true
wrap_comments = true
format_code_in_doc_comments = true
format_strings = true
format_macro_bodies = true
format_macro_matchers = true
hex_literal_case = "Preserve"
overflow_delimited_expr = true
enum_discrim_align_threshold = 0
struct_field_align_threshold = 0
match_arm_blocks = true
match_arm_leading_pipes = "Never"
force_multiline_blocks = false
fn_single_line = false
where_single_line = false
imports_indent = "Block"
imports_layout = "Mixed"
merge_derives = true
use_try_shorthand = true
use_field_init_shorthand = true
force_explicit_abi = true
condense_wildcard_suffixes = false
color = "Auto"
required_version = "1.6.0"
unstable_features = false
disable_all_formatting = false
skip_children = false
hide_parse_errors = false
error_on_line_overflow = false
error_on_unformatted = false
report_todo = "Never"
report_fixme = "Never"
ignore = []
emit_mode = "Files"
make_backup = false
```

### 4. cargo-deny Configuration (`deny.toml`)

```toml
# Dependency policy for Phase 3 migrations

[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"
notice = "warn"
ignore = []
# Exclude advisories for specific crates if needed
#ignore = [
#    "RUSTSEC-0000-0000",
#]

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Zlib",
]
deny = [
    "GPL-3.0",
    "AGPL-3.0",
]
copyleft = "warn"
allow-osi-fsf-free = "neither"
default = "deny"
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "deny"
allow-wildcard-paths = false
highlight = "all"
workspace-default-features = "allow"
external-default-features = "allow"

# Skip duplicate version check for some crates if needed
skip = []

# Specific tree of dependencies to deny
deny = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []

[sources.allow-org]
github = ["chromium", "google"]
```

### 5. sccache Configuration

#### Environment Variables

```bash
# ~/.bashrc or ~/.zshrc
export SCCACHE_DIR="/var/cache/sccache"
export SCCACHE_CACHE_SIZE="50G"
export SCCACHE_REDIS="redis://sccache-server:6379"
# Or for GCS
export SCCACHE_GCS_BUCKET="chromium-sccache"
export SCCACHE_GCS_RW_MODE="READ_WRITE"
export SCCACHE_GCS_KEY_PATH="/path/to/service-account.json"
```

#### Cargo Configuration

```toml
# .cargo/config.toml
[build]
rustc-wrapper = "/usr/local/bin/sccache"

[env]
SCCACHE_DIR = "/var/cache/sccache"
SCCACHE_CACHE_SIZE = "50G"
```

## CI/CD Templates

### 1. Main Build Workflow

```yaml
# .github/workflows/cargo-main.yml
name: Main Build (Cargo Primary)

on:
  push:
    branches: [main, release/*]
  pull_request:
  merge_group:

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  RUSTFLAGS: "-D warnings"

jobs:
  check:
    name: Check and Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
  
  build:
    name: Build (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, nightly]
        exclude:
          - os: windows-latest
            rust: nightly
    
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      
      - uses: Swatinem/rust-cache@v2
        with:
          key: ${{ matrix.os }}-${{ matrix.rust }}
      
      - name: Build workspace
        run: cargo build --workspace --all-features --all-targets
      
      - name: Build with timing
        run: cargo build --workspace --timings
        if: matrix.rust == 'stable'
      
      - name: Upload timing report
        uses: actions/upload-artifact@v3
        if: matrix.rust == 'stable'
        with:
          name: cargo-timings-${{ matrix.os }}
          path: target/cargo-timings/cargo-timing.html
  
  test:
    name: Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Run tests
        run: cargo test --workspace --all-features
      
      - name: Run doc tests
        run: cargo test --workspace --doc
  
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
      
      - name: Run audit
        run: cargo audit --deny warnings
  
  deny:
    name: Dependency Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-deny
        run: cargo install cargo-deny
      
      - name: Check licenses
        run: cargo deny check licenses
      
      - name: Check advisories
        run: cargo deny check advisories
      
      - name: Check bans
        run: cargo deny check bans
```

### 2. Performance Benchmarking Workflow

```yaml
# .github/workflows/benchmarks.yml
name: Performance Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    paths:
      - '**.rs'
      - 'Cargo.toml'
      - 'Cargo.lock'

jobs:
  benchmark:
    name: Run Benchmarks
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Run benchmarks
        run: cargo bench --workspace -- --save-baseline pr-${{ github.event.number }}
      
      - name: Compare with main
        if: github.event_name == 'pull_request'
        run: |
          cargo bench --workspace -- --baseline main --load-baseline pr-${{ github.event.number }}
      
      - name: Upload benchmark results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/output.json
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: ${{ github.event_name == 'push' }}
          alert-threshold: '115%'
          comment-on-alert: true
          fail-on-alert: false
```

### 3. Release Build Workflow

```yaml
# .github/workflows/release.yml
name: Release Build

on:
  push:
    tags:
      - 'v*.*.*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build-release:
    name: Build Release (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
    
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Build release
        run: |
          cargo build --release --target ${{ matrix.target }} --profile release-production
      
      - name: Package binaries
        run: |
          mkdir -p releases/${{ github.ref_name }}/${{ matrix.target }}
          cp target/${{ matrix.target }}/release-production/* releases/${{ github.ref_name }}/${{ matrix.target }}/
      
      - name: Create archive
        run: |
          cd releases/${{ github.ref_name }}/${{ matrix.target }}
          tar czf ../chromium-${{ github.ref_name }}-${{ matrix.target }}.tar.gz *
      
      - name: Generate checksums
        run: |
          cd releases/${{ github.ref_name }}
          sha256sum chromium-${{ github.ref_name }}-${{ matrix.target }}.tar.gz > chromium-${{ github.ref_name }}-${{ matrix.target }}.tar.gz.sha256
      
      - name: Upload release assets
        uses: actions/upload-artifact@v3
        with:
          name: release-${{ matrix.target }}
          path: releases/${{ github.ref_name }}/*.tar.gz*
  
  create-release:
    name: Create GitHub Release
    needs: build-release
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          path: releases/
      
      - name: Create release
        uses: softprops/action-gh-release@v1
        with:
          files: releases/**/*
          draft: false
          prerelease: false
```

## Build Script Templates

### Complex C++ Integration Template

```rust
// build.rs - Comprehensive C++ build example
use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    
    // 1. Build C++ code with cc crate
    build_cpp_sources();
    
    // 2. Build cxx bridge
    build_cxx_bridge();
    
    // 3. Link platform-specific libraries
    link_platform_libraries(&target_os, &target_arch);
    
    // 4. Configure rebuild triggers
    configure_rebuild_triggers();
}

fn build_cpp_sources() {
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .flag("-std=c++17")
        .warnings(true)
        .extra_warnings(true);
    
    // Platform-specific flags
    if cfg!(target_os = "linux") {
        build.flag("-fPIC");
    } else if cfg!(target_os = "windows") {
        build.flag("/EHsc");
    }
    
    // Add include directories
    build
        .include("src/cpp")
        .include("third_party/include");
    
    // Add source files
    let sources = vec![
        "src/cpp/legacy_impl.cc",
        "src/cpp/utilities.cc",
        "src/cpp/platform_specific.cc",
    ];
    
    for source in sources {
        build.file(source);
    }
    
    build.compile("legacy_cpp");
}

fn build_cxx_bridge() {
    cxx_build::bridge("src/ffi.rs")
        .file("src/cpp/bridge_impl.cc")
        .flag("-std=c++17")
        .include("src/cpp")
        .compile("ffi_bridge");
}

fn link_platform_libraries(os: &str, arch: &str) {
    match os {
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            if arch == "x86_64" {
                println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
            }
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=msvcrt");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreFoundation");
        }
        _ => {}
    }
}

fn configure_rebuild_triggers() {
    println!("cargo:rerun-if-changed=src/cpp/");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
}
```

## Testing Configuration

### Test Organization Template

```rust
// tests/integration_tests.rs
#![cfg(test)]

mod common;

use common::setup_test_environment;

#[test]
fn test_basic_functionality() {
    let env = setup_test_environment();
    // Test implementation
}

#[test]
#[ignore] // Long-running test
fn test_performance() {
    // Performance test implementation
}

// tests/common/mod.rs
pub fn setup_test_environment() -> TestEnvironment {
    TestEnvironment::new()
}

pub struct TestEnvironment {
    // Common test setup
}

impl TestEnvironment {
    pub fn new() -> Self {
        // Initialize test environment
        Self {}
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Cleanup
    }
}
```

### Benchmark Template

```rust
// benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chromium_component::*;

fn benchmark_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("operation");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| {
                    perform_operation(black_box(size))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_compared_to_cpp(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_vs_cpp");
    
    group.bench_function("rust_implementation", |b| {
        b.iter(|| rust_implementation(black_box(1000)))
    });
    
    group.bench_function("cpp_implementation", |b| {
        b.iter(|| cpp_implementation(black_box(1000)))
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_operation, benchmark_compared_to_cpp);
criterion_main!(benches);
```

## Automation Scripts

### Migration Preparation Script

```bash
#!/bin/bash
# tools/prepare_migration.sh

set -e

COMPONENT=$1

if [ -z "$COMPONENT" ]; then
    echo "Usage: $0 <component_path>"
    exit 1
fi

echo "Preparing migration for $COMPONENT..."

# 1. Generate Cargo.toml
echo "Generating Cargo.toml..."
python3 tools/cargo_migration/gn_to_cargo.py \
    "$COMPONENT/BUILD.gn" \
    --target "$(basename $COMPONENT)" \
    -o "$COMPONENT/Cargo.toml"

# 2. Create build.rs template if needed
if grep -q "cxx\|cc\|cmake" "$COMPONENT/BUILD.gn"; then
    echo "Creating build.rs template..."
    cp tools/templates/build.rs.template "$COMPONENT/build.rs"
fi

# 3. Add to workspace
echo "Adding to workspace..."
sed -i "/members = \[/a \ \ \ \ \"$COMPONENT\"," Cargo.toml

# 4. Initial build attempt
echo "Attempting initial build..."
cargo check -p "$(basename $COMPONENT)" || true

echo "Migration preparation complete!"
echo "Next steps:"
echo "1. Review generated Cargo.toml"
echo "2. Customize build.rs if created"
echo "3. Fix compilation errors"
echo "4. Run tests with: cargo test -p $(basename $COMPONENT)"
```

### Validation Script

```bash
#!/bin/bash
# tools/validate_migration.sh

set -e

COMPONENT=$1

if [ -z "$COMPONENT" ]; then
    echo "Usage: $0 <component_name>"
    exit 1
fi

echo "Validating migration for $COMPONENT..."

# 1. Build check
echo "1. Checking build..."
cargo build -p "$COMPONENT" --all-targets

# 2. Test check
echo "2. Running tests..."
cargo test -p "$COMPONENT"

# 3. Clippy check
echo "3. Running clippy..."
cargo clippy -p "$COMPONENT" -- -D warnings

# 4. Format check
echo "4. Checking formatting..."
cargo fmt -p "$COMPONENT" -- --check

# 5. Security audit
echo "5. Running security audit..."
cargo audit

# 6. Benchmark (if exists)
if [ -d "benches" ]; then
    echo "6. Running benchmarks..."
    cargo bench -p "$COMPONENT" --no-run
fi

echo "✓ All validation checks passed!"
```

## Summary

These templates provide:
- **IDE Configuration**: VSCode, CLion setup for optimal Rust development
- **Linting/Formatting**: Clippy and rustfmt configurations
- **Dependency Management**: cargo-deny policies
- **Build Optimization**: sccache setup
- **CI/CD**: Complete GitHub Actions workflows
- **Build Scripts**: Complex C++ integration examples
- **Testing**: Integration and benchmark templates
- **Automation**: Migration preparation and validation scripts

Use these as starting points and customize for specific component needs.
