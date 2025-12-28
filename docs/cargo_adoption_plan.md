# Cargo Build System Adoption Plan

[TOC]

## Executive Summary

This document outlines a strategic plan to adopt Cargo and standard Rust tooling as the primary build system for the Chromium/Crustonium project, transitioning away from the current GN/Ninja-based build system. This is an ambitious, multi-year effort that must maintain backward compatibility and support incremental migration.

**Current State**: The project uses GN (Generate Ninja) as the meta-build system, with Ninja as the underlying build executor. Rust is integrated via GN templates (`rust_static_library.gni`, `cargo_crate.gni`).

**Goal**: Transition to Cargo as the primary build system, leveraging the Rust ecosystem's standard tools while maintaining the ability to build C++ code during the transition period.

**Timeline**: 3-5 years for complete transition, with hybrid mode for extended period.

## Vision and Objectives

### Primary Objectives

1. **Adopt Cargo as Primary Build System**: Use Cargo's dependency management, build orchestration, and tooling
2. **Leverage Rust Ecosystem**: Enable use of standard Rust tools (cargo-clippy, cargo-fmt, cargo-audit, etc.)
3. **Simplify Build Process**: Reduce complexity compared to GN/Ninja system
4. **Improve Developer Experience**: Better IDE integration, faster iteration, clearer dependency management
5. **Maintain Compatibility**: Support building existing C++ code during transition

### Non-Objectives

- Immediate complete rewrite of the build system (this will be gradual)
- Abandoning C++ immediately (hybrid builds will be supported for years)
- Breaking existing developer workflows without providing migration path
- Losing platform support (must support all current target platforms)

## Challenges and Constraints

### Technical Challenges

1. **C++ Integration**: Cargo is designed for Rust; integrating C++ builds is non-standard
2. **Complex Dependencies**: Chromium has intricate dependency graphs across languages
3. **Platform Support**: Must support Windows, macOS, Linux, ChromeOS, Android, iOS
4. **Build Performance**: GN/Ninja is highly optimized; Cargo must match or exceed
5. **Custom Build Steps**: Many custom build steps (code generation, resource compilation, etc.)
6. **Monorepo Structure**: Chromium is a large monorepo with complex workspace structure

### Organizational Challenges

1. **Team Learning Curve**: Developers must learn Cargo and Rust tooling
2. **Infrastructure Changes**: CI/CD systems must adapt
3. **Tooling Migration**: Many tools are GN-aware and need updates
4. **Resistance to Change**: Large codebase with established workflows
5. **Coordination**: Changes affect all teams and components

## Migration Strategy

### Phase 1: Foundation and Preparation (Months 1-12)

**Goal**: Build foundational support for Cargo while maintaining GN/Ninja

#### 1.1 Research and Prototyping

- [ ] **Evaluate C++ build integration options**
  - Research `cc` crate for building C/C++ from Cargo
  - Evaluate `cmake` crate for CMake integration
  - Prototype hybrid Cargo+CMake builds
  - Test `cargo-make` and other task runners for complex builds

- [ ] **Prototype workspace structure**
  - Design Cargo workspace layout for monorepo
  - Define crate boundaries and dependencies
  - Test virtual manifests for workspace organization
  - Prototype dependency vendoring strategy

- [ ] **Performance benchmarking**
  - Measure GN/Ninja build times (clean, incremental)
  - Prototype Cargo builds and measure performance
  - Identify bottlenecks and optimization opportunities
  - Test build caching strategies (sccache, etc.)

#### 1.2 Tooling Development

- [ ] **Create GN-to-Cargo translation tools**
  - Build parser for BUILD.gn files
  - Generate Cargo.toml from BUILD.gn
  - Handle cross-language dependencies
  - Support conditional compilation (platforms, features)

- [ ] **Develop hybrid build system**
  - Support both GN and Cargo in parallel
  - Create wrapper scripts for transparent switching
  - Implement compatibility shims
  - Ensure identical build outputs

- [ ] **CI/CD Integration**
  - Update build bots to support Cargo
  - Implement Cargo caching in CI
  - Set up cargo-deny for dependency auditing
  - Integrate cargo-audit for security scanning

#### 1.3 Documentation and Training

- [ ] **Create comprehensive guides**
  - Cargo basics for Chromium developers
  - Migration guide from GN to Cargo
  - Best practices for workspace management
  - Troubleshooting guide for common issues

- [ ] **Pilot program**
  - Select 3-5 small components for early migration
  - Document lessons learned
  - Refine migration process
  - Build champion network

### Phase 2: Incremental Migration (Months 13-30)

**Goal**: Migrate Rust-heavy components to Cargo, establish hybrid build patterns

#### 2.1 Component Migration Priority

**Tier 1: Pure Rust Components** (Months 13-18)
- [ ] Testing infrastructure (`testing/rust_gtest_interop/`)
- [ ] Build tools (`tools/crates/gnrt/`)
- [ ] QR code generator (already pure Rust with FFI)
- [ ] Rust utilities and libraries

**Tier 2: Rust with C++ FFI** (Months 19-24)
- [ ] Media filters (`media/filters/symphonia_glue.rs`)
- [ ] Bluetooth parser (`device/bluetooth/bluez/ble_scan_parser/`)
- [ ] Payment validation (`components/facilitated_payments/`)
- [ ] Data import utilities (`components/user_data_importer/`)

**Tier 3: Mixed Components** (Months 25-30)
- [ ] Components with significant C++ that will be ported to Rust
- [ ] New components developed in Rust
- [ ] Gradually expanding Rust coverage

#### 2.2 Workspace Structure Design

```
chromium/
├── Cargo.toml                 # Workspace root manifest
├── Cargo.lock                 # Locked dependencies
├── .cargo/
│   └── config.toml           # Cargo configuration
├── crates/                   # First-party Rust crates
│   ├── base/                 # Base utilities (Rust equivalent of //base)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── qr_code_generator/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── media_filters/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── build.rs         # Build script for C++ integration
│   └── ...
├── third_party/
│   └── rust/                 # Vendored dependencies
├── cpp/                      # C++ code (during transition)
│   ├── CMakeLists.txt       # CMake for C++ builds
│   └── ...
└── build/                    # Build system support
    ├── cargo/               # Cargo-specific build support
    └── cmake/               # CMake modules for C++ integration
```

#### 2.3 Hybrid Build System

**Cargo Workspace Manifest** (`Cargo.toml`):
```toml
[workspace]
resolver = "2"

members = [
    "crates/base",
    "crates/qr_code_generator",
    "crates/media_filters",
    # ... all first-party crates
]

# Workspace-wide dependencies
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
log = "0.4"
cxx = "1.0"

# Workspace configuration
[workspace.package]
edition = "2021"
license = "BSD-3-Clause"
repository = "https://github.com/rexlunae/crustonium"

# Profile optimization
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1

[profile.dev]
opt-level = 0
debug = true

# For production builds
[profile.production]
inherits = "release"
lto = "fat"
panic = "abort"
strip = true
```

**Example Component Cargo.toml**:
```toml
# crates/qr_code_generator/Cargo.toml
[package]
name = "chromium-qr-code-generator"
version = "1.0.0"
edition.workspace = true
license.workspace = true

[dependencies]
qr_code = "2.0"
cxx = "1.0"

# Workspace dependencies
serde.workspace = true
thiserror.workspace = true

[build-dependencies]
cxx-build = "1.0"

# For components with C++ integration
[lib]
crate-type = ["staticlib", "rlib"]
```

**Build Script for C++ Integration** (`build.rs`):
```rust
// crates/qr_code_generator/build.rs
fn main() {
    // cxx bridge compilation
    cxx_build::bridge("src/ffi.rs")
        .file("src/wrapper.cc")
        .flag_if_supported("-std=c++17")
        .compile("qr_code_generator_cxx");
    
    // Tell cargo to rerun if C++ files change
    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    
    // Link against C++ stdlib
    println!("cargo:rustc-link-lib=stdc++");
}
```

#### 2.4 Building C++ Dependencies

**Option A: Using `cc` crate** (for simple C++ code):
```rust
// build.rs
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/implementation.cc")
        .flag("-std=c++17")
        .compile("cpp_lib");
}
```

**Option B: Using CMake** (for complex C++ builds):
```rust
// build.rs
fn main() {
    let dst = cmake::Config::new("cpp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();
    
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cpp_component");
}
```

**Option C: Cargo + CMake hybrid**:
```toml
# Cargo.toml
[package]
name = "chromium"
version = "1.0.0"

# Use cargo-make for complex build orchestration
[dependencies]
# Rust dependencies

[build-dependencies]
cmake = "0.1"
cc = "1.0"
```

```toml
# Makefile.toml (for cargo-make)
[tasks.build-cpp]
command = "cmake"
args = ["--build", "build/cpp", "--parallel"]

[tasks.build-rust]
command = "cargo"
args = ["build", "--workspace"]

[tasks.build]
dependencies = ["build-cpp", "build-rust"]

[tasks.test]
command = "cargo"
args = ["test", "--workspace"]
```

### Phase 3: Comprehensive Migration (Months 31-48)

**Goal**: Majority of builds use Cargo, GN/Ninja in maintenance mode

#### 3.1 Core Component Migration

- [ ] **Migrate major subsystems**
  - Network stack components
  - Storage and persistence
  - Platform abstraction layers
  - Resource management

- [ ] **Establish C++ build patterns**
  - Standardize C++ integration via CMake
  - Create reusable build scripts
  - Document best practices
  - Optimize C++ build performance

#### 3.2 Developer Experience

- [ ] **IDE Integration**
  - Ensure rust-analyzer works seamlessly
  - VSCode configuration for workspace
  - CLion/IntelliJ Rust plugin support
  - Debugging configuration

- [ ] **Standard Rust Tooling**
  - Enable `cargo clippy` for linting
  - Use `cargo fmt` for formatting
  - Integrate `cargo-deny` for dependency auditing
  - Set up `cargo-audit` for security
  - Use `cargo-outdated` for dependency management

- [ ] **Build Optimization**
  - Implement distributed builds (sccache, cachepot)
  - Optimize dependency tree
  - Reduce compilation units
  - Implement smart rebuild detection

#### 3.3 Infrastructure Updates

- [ ] **CI/CD Migration**
  - Primary build path uses Cargo
  - GN/Ninja as fallback/compatibility
  - Performance tracking and comparison
  - Binary size monitoring

- [ ] **Release Process**
  - Update release scripts
  - Binary packaging from Cargo
  - Debug symbol handling
  - Cross-compilation setup

### Phase 4: Completion and Optimization (Months 49-60)

**Goal**: Cargo as sole build system, GN/Ninja deprecated

#### 4.1 Final Migration

- [ ] **Migrate remaining components**
  - Legacy C++ components (if not yet ported to Rust)
  - Platform-specific code
  - Third-party integrations

- [ ] **Remove GN/Ninja**
  - Archive GN/Ninja build files
  - Remove build system infrastructure
  - Update documentation
  - Celebrate! 🎉

#### 4.2 Optimization and Polish

- [ ] **Build Performance**
  - Profile and optimize build times
  - Minimize dependencies
  - Optimize feature flags
  - Tune parallel compilation

- [ ] **Developer Experience**
  - Gather feedback and iterate
  - Improve error messages
  - Enhance tooling integration
  - Streamline workflows

## Technical Implementation Details

### Cargo Configuration

**Workspace `.cargo/config.toml`**:
```toml
[build]
# Use all available CPU cores
jobs = 0

# Incremental compilation for faster rebuilds
incremental = true

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-msvc]
linker = "lld-link.exe"

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Cargo registry configuration
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "third_party/rust"

# Build cache configuration
[build]
target-dir = "target"

# Network configuration
[net]
git-fetch-with-cli = true

# Alias for common commands
[alias]
b = "build"
t = "test"
r = "run"
c = "check"
```

### Dependency Management

**Strategy for Third-party Dependencies**:

1. **Vendor all dependencies** (following current Chromium practice):
   ```bash
   cargo vendor third_party/rust
   ```

2. **Use workspace dependencies** for version uniformity:
   ```toml
   [workspace.dependencies]
   serde = { version = "1.0", features = ["derive"] }
   
   # In member crate:
   [dependencies]
   serde.workspace = true
   ```

3. **Dependency auditing**:
   ```bash
   cargo deny check licenses
   cargo deny check advisories
   cargo deny check bans
   ```

4. **Update process**:
   ```bash
   cargo update                    # Update within semver
   cargo upgrade                   # Update to latest (requires cargo-edit)
   cargo vendor third_party/rust   # Re-vendor
   ```

### Cross-compilation

**Platform Support**:
```toml
# .cargo/config.toml

[target.aarch64-linux-android]
linker = "aarch64-linux-android-clang"
ar = "aarch64-linux-android-ar"

[target.aarch64-apple-ios]
linker = "rust-lld"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

# Add all required targets
```

**Build commands**:
```bash
# Build for Android
cargo build --target aarch64-linux-android --release

# Build for iOS
cargo build --target aarch64-apple-ios --release

# Build for Windows from Linux
cargo build --target x86_64-pc-windows-gnu --release
```

### Build Scripts and Code Generation

**Complex build script example**:
```rust
// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Build C++ code
    build_cpp_code();
    
    // 2. Generate code from IDL
    generate_from_idl();
    
    // 3. Process resources
    process_resources();
    
    // 4. Configure platform-specific settings
    configure_platform();
}

fn build_cpp_code() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++17")
        .files(&[
            "cpp/implementation.cc",
            "cpp/utilities.cc",
        ]);
    
    if cfg!(target_os = "windows") {
        build.flag("/EHsc");
    }
    
    build.compile("cpp_component");
}

fn generate_from_idl() {
    // Use existing code generators
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Example: Generate bindings
    bindgen::Builder::default()
        .header("cpp/api.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn process_resources() {
    // Compile resources, generate data files, etc.
    println!("cargo:rerun-if-changed=resources/");
}

fn configure_platform() {
    // Platform-specific configuration
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "linux" => println!("cargo:rustc-cfg=unix_platform"),
        "macos" => println!("cargo:rustc-cfg=unix_platform"),
        "windows" => println!("cargo:rustc-cfg=windows_platform"),
        _ => {}
    }
}
```

### Testing Integration

**Cargo test structure**:
```toml
# Cargo.toml
[package]
name = "chromium-component"

[dev-dependencies]
criterion = "0.5"      # For benchmarking
proptest = "1.0"       # Property-based testing
mockall = "0.12"       # Mocking

[[test]]
name = "integration"
path = "tests/integration_test.rs"

[[bench]]
name = "performance"
harness = false
```

**Running tests**:
```bash
# All tests
cargo test --workspace

# Specific package
cargo test -p chromium-qr-code-generator

# Integration tests only
cargo test --test integration

# Benchmarks
cargo bench

# With test coverage
cargo tarpaulin --workspace
```

### Continuous Integration

**GitHub Actions example**:
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, nightly]
    
    steps:
    - uses: actions/checkout@v3
    
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - uses: Swatinem/rust-cache@v2
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Clippy
      run: cargo clippy --workspace --all-targets -- -D warnings
    
    - name: Build
      run: cargo build --workspace --all-targets
    
    - name: Test
      run: cargo test --workspace
    
    - name: Audit
      run: |
        cargo install cargo-audit
        cargo audit
```

## Migration Tools

### GN-to-Cargo Converter

**Tool to translate BUILD.gn to Cargo.toml**:

```python
# tools/build/gn_to_cargo.py
import json
import sys
import toml

def convert_gn_to_cargo(build_gn_path):
    """Convert BUILD.gn to Cargo.toml"""
    
    # Parse GN file (simplified)
    gn_data = parse_gn_file(build_gn_path)
    
    cargo_toml = {
        "package": {
            "name": gn_data["target_name"].replace(":", "-"),
            "version": "1.0.0",
            "edition": "2021",
        },
        "dependencies": {},
        "build-dependencies": {},
    }
    
    # Convert dependencies
    for dep in gn_data.get("deps", []):
        if dep.startswith("//third_party/rust/"):
            crate_name = extract_crate_name(dep)
            cargo_toml["dependencies"][crate_name] = "*"
    
    # Convert sources
    if "sources" in gn_data:
        cargo_toml["lib"] = {
            "path": gn_data["sources"][0]
        }
    
    # Write Cargo.toml
    with open("Cargo.toml", "w") as f:
        toml.dump(cargo_toml, f)

def parse_gn_file(path):
    # Simplified parser - real implementation would use GN's JSON output
    pass

def extract_crate_name(gn_dep):
    # Extract crate name from GN dependency path
    pass

if __name__ == "__main__":
    convert_gn_to_cargo(sys.argv[1])
```

### Parallel Build Support Script

```bash
#!/bin/bash
# build.sh - Wrapper script for hybrid builds

BUILD_SYSTEM="${BUILD_SYSTEM:-cargo}"

case "$BUILD_SYSTEM" in
    cargo)
        echo "Building with Cargo..."
        cargo build --workspace --release
        ;;
    gn)
        echo "Building with GN/Ninja..."
        gn gen out/Default
        ninja -C out/Default
        ;;
    hybrid)
        echo "Building with hybrid system..."
        # Build Rust components with Cargo
        cargo build --workspace --release
        
        # Build remaining C++ with GN/Ninja
        gn gen out/Default --args='use_cargo_rust=true'
        ninja -C out/Default
        ;;
    *)
        echo "Unknown build system: $BUILD_SYSTEM"
        exit 1
        ;;
esac
```

## Risk Management

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Build performance regression | High | Medium | Benchmark early, optimize incrementally, parallel builds |
| C++ integration complexity | High | High | Prototype thoroughly, use standard tools (cc, cmake crates) |
| Platform support gaps | High | Low | Test on all platforms early, engage Rust team for support |
| Binary size increase | Medium | Medium | Monitor size, use LTO, strip symbols, optimize dependencies |
| Tooling incompatibility | Medium | Medium | Maintain GN/Ninja in parallel during transition |
| Developer productivity loss | High | Medium | Comprehensive training, gradual migration, good documentation |

### Organizational Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Team resistance | High | High | Clear communication, demonstrate benefits, voluntary early adoption |
| Learning curve too steep | Medium | Medium | Training program, documentation, champions, pair programming |
| Migration fatigue | Medium | High | Celebrate wins, show progress, maintain momentum |
| Resource constraints | High | Medium | Secure dedicated resources, prioritize work, automate where possible |
| Loss of institutional knowledge | Medium | Low | Document thoroughly, knowledge transfer sessions |

## Success Metrics

### Build Performance

- **Build time** (clean build): Target ≤ current GN/Ninja time
- **Incremental build time**: Target < current GN/Ninja time (Cargo should excel here)
- **CI/CD time**: Target ≤ current time
- **Cache hit rate**: Target > 80%

### Developer Experience

- **Time to onboard** new developers: Target < 1 day for basic productivity
- **Build system satisfaction**: Survey score > 4/5
- **IDE integration quality**: "Works out of the box" on primary IDEs
- **Documentation completeness**: All common tasks documented

### Code Quality

- **Dependency freshness**: Update dependencies quarterly
- **Security vulnerability count**: Zero unpatched critical vulnerabilities
- **License compliance**: 100% compliant
- **Code coverage**: Maintain or improve current levels

### Migration Progress

- **Components migrated**: Track percentage of components on Cargo
- **Lines of code**: Track Rust vs C++ ratio
- **Build system usage**: Track Cargo vs GN/Ninja usage in CI

## Communication and Training

### Training Program

**Phase 1: Fundamentals**
- Cargo basics workshop (2 hours)
- Workspace management (1 hour)
- Cargo.toml structure and dependencies (1 hour)

**Phase 2: Advanced Topics**
- C++ integration with Cargo (2 hours)
- Build scripts and code generation (2 hours)
- Cross-compilation (1 hour)
- Performance optimization (1 hour)

**Phase 3: Migration Practice**
- Hands-on migration workshop (4 hours)
- Build system debugging (1 hour)
- CI/CD integration (1 hour)

### Documentation

- **User Guide**: "Building Chromium with Cargo"
- **Migration Guide**: "Migrating from GN to Cargo"
- **Reference**: "Cargo Configuration for Chromium"
- **Troubleshooting**: Common issues and solutions
- **Best Practices**: Patterns and anti-patterns

### Communication Plan

- **Monthly**: Build system migration newsletter
- **Quarterly**: Town hall on progress and upcoming changes
- **Weekly**: Office hours for questions and support
- **Ongoing**: Slack #cargo-migration channel

## Governance

### Decision Authority

- **Build System Working Group**: Makes technical decisions
- **Architecture Team**: Approves major architectural changes
- **Leadership**: Approves resource allocation and timeline

### Review Process

- **Build System Changes**: Requires Build System WG approval
- **Migration CLs**: Requires component owner + build system expert review
- **Tooling Changes**: Requires Build System WG + affected team approval

## Timeline and Milestones

### Year 1: Foundation
- **Q1**: Research complete, prototype validated
- **Q2**: Tools developed, documentation started
- **Q3**: Pilot migrations (3-5 components)
- **Q4**: Hybrid build system operational

### Year 2-3: Migration
- **Q1-Q4 (Year 2)**: Migrate Rust components and Rust-heavy mixed components
- **Q1-Q4 (Year 3)**: Migrate remaining components, optimize build

### Year 4-5: Completion
- **Q1-Q2 (Year 4)**: Final migrations, GN/Ninja in maintenance mode
- **Q3-Q4 (Year 4)**: Deprecate GN/Ninja, Cargo is primary
- **Year 5**: Optimization, polish, removal of legacy build system

## Conclusion

Migrating from GN/Ninja to Cargo as the primary build system is a significant undertaking that will take 3-5 years. However, the benefits are substantial:

- **Standardization**: Leverage the Rust ecosystem's standard tooling
- **Simplification**: Reduce build system complexity
- **Developer Experience**: Better IDE integration, faster iteration
- **Ecosystem Access**: Easy access to the Rust crate ecosystem
- **Future-Proofing**: Align with the direction of increased Rust adoption

Success requires:
- Strong leadership support and dedicated resources
- Comprehensive tooling and automation
- Thorough documentation and training
- Patience and pragmatic, incremental execution
- Continuous communication and feedback loops

This plan provides a roadmap for this transition while managing risk and maintaining project velocity throughout the migration period.

---

**See Also**:
- [Rust Adoption Plan](../rust_adoption_plan.md)
- [Migration Guide](../rust/migration_guide.md)
- [Quick Reference](../rust/quick_reference.md)
