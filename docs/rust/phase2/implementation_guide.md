# Phase 2: Incremental Migration - Implementation Guide

**Status**: Ready for Execution  
**Duration**: Months 13-30 (18 months)  
**Goal**: Migrate Rust-heavy components to Cargo, establish hybrid build patterns

[TOC]

## Overview

Phase 2 transitions from planning and prototyping to actual component migration. This phase establishes the production hybrid build system and migrates components in priority order while maintaining full backward compatibility with GN/Ninja.

### Phase 2 Sub-Phases

**2.1 Component Migration Priority** (Months 13-30)
- Tier 1: Pure Rust Components (Months 13-18)
- Tier 2: Rust with C++ FFI (Months 19-24)
- Tier 3: Mixed Components (Months 25-30)

**2.2 Workspace Structure Design** (Ongoing)
- Production workspace architecture
- Crate organization
- Dependency management

**2.3 Hybrid Build System** (Months 13-15)
- Cargo + GN/Ninja parallel builds
- Build system integration
- CI/CD updates

**2.4 Building C++ Dependencies** (Ongoing)
- cc crate integration
- CMake integration
- Build script patterns

## Component Migration Tiers

### Tier 1: Pure Rust Components (Months 13-18)

**Target Components**:
1. **Testing Infrastructure** (`testing/rust_gtest_interop/`)
   - Status: Ready for migration
   - Risk: Low
   - Impact: High (enables better Rust testing)
   
2. **Build Tools** (`tools/crates/gnrt/`)
   - Status: Ready for migration  
   - Risk: Very Low (pure Rust binary)
   - Impact: Medium (improves developer workflow)
   
3. **QR Code Generator** (`components/qr_code_generator/`)
   - Status: Needs FFI adjustment
   - Risk: Low-Medium
   - Impact: Medium (validates FFI patterns)

**Migration Process** (6-8 weeks per component):
1. **Preparation** (Week 1)
   - Document current state
   - Review dependencies
   - Plan FFI boundaries
   
2. **Cargo Setup** (Week 2)
   - Generate Cargo.toml with gn_to_cargo.py
   - Add to workspace
   - Configure build.rs if needed
   
3. **Build Validation** (Week 3-4)
   - Build with Cargo
   - Fix compilation errors
   - Validate outputs
   
4. **Testing** (Week 5)
   - Run Cargo tests
   - Compare with GN test results
   - Fix test failures
   
5. **Integration** (Week 6)
   - Update BUILD.gn for hybrid support
   - Update CI/CD
   - Document changes
   
6. **Validation** (Week 7-8)
   - Performance testing
   - Team validation
   - Migration report

**Success Criteria**:
- ✅ Component builds with `cargo build`
- ✅ All tests pass with `cargo test`  
- ✅ Build times comparable or better than GN
- ✅ No functionality regressions
- ✅ Documentation updated

### Tier 2: Rust with C++ FFI (Months 19-24)

**Target Components**:
1. **Media Filters** (`media/filters/symphonia_glue.rs`)
   - Complex FFI with media codecs
   - Risk: Medium
   
2. **Bluetooth Parser** (`device/bluetooth/bluez/ble_scan_parser/`)
   - Parsing with C++ interop
   - Risk: Medium
   
3. **Payment Validation** (`components/facilitated_payments/`)
   - Security-sensitive component
   - Risk: Medium-High
   
4. **Data Import Utilities** (`components/user_data_importer/`)
   - Data processing with FFI
   - Risk: Medium

**Additional Challenges**:
- Complex C++ dependencies
- Extensive FFI surface
- Performance requirements
- Security considerations

**Migration Approach**:
- Use `cc` crate for simple C++ compilation
- Use `cxx` bridge for type-safe FFI
- Consider `cmake` crate for complex C++ builds
- Extensive testing required

### Tier 3: Mixed Components (Months 25-30)

**Target Components**:
- Components with significant C++ that will be ported to Rust
- New components developed in Rust
- Gradually expanding Rust coverage

**Strategy**:
- Incremental Rust adoption within components
- Parallel implementation (Rust + C++)
- Feature flags for gradual rollout
- A/B testing for validation

## Workspace Structure

### Production Workspace Architecture

```
chromium/
├── Cargo.toml                     # Workspace root manifest
├── Cargo.lock                     # Locked dependencies (COMMIT THIS)
├── .cargo/
│   └── config.toml               # Cargo configuration
│
├── crates/                       # First-party Rust crates
│   ├── base/                     # Base utilities (future)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── qr_code_generator/        # Tier 1 migration
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── build.rs             # C++ integration
│   ├── rust_gtest_interop/       # Tier 1 migration
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── build_tools/              # Tier 1 migration  
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── media_filters/            # Tier 2 migration
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── build.rs
│   └── ...
│
├── prototypes/                   # Phase 1 prototypes (kept for reference)
│   ├── cargo_cpp_integration/
│   └── workspace_structure_test/
│
├── third_party/
│   └── rust/                     # Vendored dependencies (optional)
│
├── components/                   # Original component locations
│   └── qr_code_generator/       # Original code (maintained for GN)
│       ├── BUILD.gn
│       ├── Cargo.toml           # Symlink to crates/qr_code_generator/Cargo.toml
│       └── ...
│
└── tools/
    └── cargo_migration/          # Migration tools
        ├── gn_to_cargo.py
        └── hybrid_build.sh
```

### Workspace Cargo.toml (Phase 2)

```toml
# Cargo Workspace for Chromium/Crustonium
# Phase 2: Incremental Migration

[workspace]
resolver = "2"

members = [
    # Phase 1 prototypes (kept for reference)
    "prototypes/cargo_cpp_integration",
    "prototypes/workspace_structure_test",
    
    # Phase 2: Tier 1 Components (Pure Rust)
    "crates/qr_code_generator",
    "crates/rust_gtest_interop",
    "crates/build_tools",
    
    # Phase 2: Tier 2 Components (Rust + C++ FFI) - Added progressively
    # "crates/media_filters",
    # "crates/bluetooth_parser",
    # "crates/payment_validation",
]

exclude = [
    "third_party",
    "out",
    "target",
    "build",
]

[workspace.dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
log = "0.4"

# FFI and C++ integration
cxx = "1.0"
cc = "1.0"
cmake = "0.1"

# Build dependencies
cxx-build = "1.0"

# Testing
criterion = "0.5"
googletest = "0.11"

# Component-specific dependencies
qr_code = "2.0"
symphonia = "0.5"  # For media filters

[workspace.package]
version = "1.0.0"
edition = "2021"
license = "BSD-3-Clause"
repository = "https://github.com/rexlunae/crustonium"

[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "unpacked"

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

## Hybrid Build System

### Build System Coexistence

During Phase 2, both build systems operate in parallel:

**Cargo Builds**:
```bash
# Build specific component
cargo build -p chromium-qr-code-generator

# Build all migrated components
cargo build --workspace

# Run tests
cargo test --workspace
```

**GN/Ninja Builds** (Still primary):
```bash
# Traditional build (includes Cargo-built components)
ninja -C out/Default chrome

# The GN build calls Cargo internally for migrated components
```

**Hybrid Build Script**:
```bash
# Build everything with both systems
./tools/cargo_migration/hybrid_build.sh --system hybrid

# Just Cargo components
./tools/cargo_migration/hybrid_build.sh --system cargo

# Just GN (legacy)
./tools/cargo_migration/hybrid_build.sh --system gn
```

### BUILD.gn Integration

For migrated components, update BUILD.gn to support Cargo:

```python
# components/qr_code_generator/BUILD.gn

import("//build/rust/rust_static_library.gni")
import("//build/rust/cargo_crate.gni")

# New Cargo-based build (Phase 2+)
cargo_crate("qr_code_generator") {
  crate_root = "../../crates/qr_code_generator/Cargo.toml"
  output_name = "qr_code_generator"
  
  # Features to enable
  features = []
  
  # Dependencies
  deps = [
    "//third_party/rust:qr_code",
  ]
}

# Legacy GN build (fallback, Phase 2)
rust_static_library("qr_code_generator_legacy") {
  crate_root = "qr_code_generator_ffi_glue.rs"
  sources = [ "qr_code_generator_ffi_glue.rs" ]
  
  deps = [
    "//third_party/rust:qr_code",
  ]
  
  # Deprecated - remove in Phase 3
  visibility = [ ":*" ]
}

# Use Cargo build by default
group("qr_code_generator_default") {
  public_deps = [ ":qr_code_generator" ]
}
```

### CI/CD Integration

Update `.github/workflows/cargo-ci.yml` to build migrated components:

```yaml
jobs:
  build-migrated-components:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
        
      - name: Build Tier 1 components
        run: |
          cargo build -p chromium-qr-code-generator
          cargo build -p chromium-rust-gtest-interop
          cargo build -p chromium-build-tools
          
      - name: Test Tier 1 components
        run: |
          cargo test --workspace --exclude prototypes
          
      - name: Benchmark
        run: cargo bench --no-fail-fast
```

## Migration Playbook

### Step-by-Step Migration Process

#### 1. Component Selection

**Criteria**:
- Rust-heavy or pure Rust
- Well-defined boundaries
- Good test coverage
- Not on critical path initially

**Decision Tree**:
```
Is component pure Rust?
├─ Yes → Tier 1 (Months 13-18)
└─ No → Has C++ FFI?
    ├─ Yes → Tier 2 (Months 19-24)
    └─ No → Mixed → Tier 3 (Months 25-30)
```

#### 2. Pre-Migration Checklist

- [ ] Component identified and approved
- [ ] Migration owner assigned
- [ ] Current build times documented
- [ ] Test coverage verified (>70%)
- [ ] Dependencies catalogued
- [ ] FFI boundaries documented
- [ ] Migration timeline agreed (6-8 weeks)

#### 3. Migration Execution

**Week 1: Setup**
```bash
# Generate Cargo.toml
python3 tools/cargo_migration/gn_to_cargo.py \
  components/COMPONENT/BUILD.gn \
  --target TARGET_NAME \
  --output crates/COMPONENT/Cargo.toml

# Create crate structure
mkdir -p crates/COMPONENT/src
cp components/COMPONENT/*.rs crates/COMPONENT/src/

# Add to workspace
# Edit Cargo.toml, add "crates/COMPONENT" to members
```

**Week 2-3: Build**
```bash
# Try building
cargo build -p chromium-COMPONENT

# Fix errors iteratively
# Common issues:
# - Path adjustments
# - Dependency versions
# - FFI configuration
```

**Week 4: Testing**
```bash
# Run tests
cargo test -p chromium-COMPONENT

# Compare with GN
ninja -C out/Default component_tests
out/Default/component_tests
```

**Week 5: Integration**
```bash
# Update BUILD.gn
# Add cargo_crate() target
# Keep legacy build as fallback

# Test hybrid build
./tools/cargo_migration/hybrid_build.sh
```

**Week 6-7: Validation**
```bash
# Performance testing
time cargo build -p chromium-COMPONENT  # Clean
time cargo build -p chromium-COMPONENT  # Incremental

# Team validation
# Other developers build and test
# Collect feedback

# CI validation
# Ensure CI passes
```

**Week 8: Documentation**
- Write migration report
- Update component README
- Document lessons learned
- Present to team

#### 4. Post-Migration Checklist

- [ ] Component builds with Cargo
- [ ] All tests passing
- [ ] BUILD.gn updated for hybrid support
- [ ] CI passing
- [ ] Documentation updated
- [ ] Migration report written
- [ ] Team validated
- [ ] Performance acceptable

## C++ Integration Patterns

### Pattern 1: Simple C++ Compilation (cc crate)

**Use case**: Simple C++ files, no complex build

**build.rs**:
```rust
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/implementation.cc")
        .file("cpp/utilities.cc")
        .flag("-std=c++17")
        .flag_if_supported("-Wall")
        .compile("cpp_lib");
    
    println!("cargo:rerun-if-changed=cpp/");
    println!("cargo:rustc-link-lib=stdc++");
}
```

### Pattern 2: cxx Bridge (Type-safe FFI)

**Use case**: Type-safe C++/Rust interop

**src/ffi.rs**:
```rust
#[cxx::bridge(namespace = "my_component")]
mod ffi {
    extern "C++" {
        include!("my_component/wrapper.h");
        
        fn cpp_function(input: &[u8]) -> String;
    }
    
    extern "Rust" {
        fn rust_function(data: &str) -> Vec<u8>;
    }
}
```

**build.rs**:
```rust
fn main() {
    cxx_build::bridge("src/ffi.rs")
        .file("src/wrapper.cc")
        .flag("-std=c++17")
        .compile("ffi_bridge");
    
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cc");
}
```

### Pattern 3: CMake Integration

**Use case**: Complex C++ projects with CMake

**build.rs**:
```rust
fn main() {
    let dst = cmake::Config::new("cpp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=my_cpp_component");
}
```

**cpp/CMakeLists.txt**:
```cmake
cmake_minimum_required(VERSION 3.15)
project(my_cpp_component)

add_library(my_cpp_component STATIC
    implementation.cc
    utilities.cc
)

target_compile_features(my_cpp_component PUBLIC cxx_std_17)
```

## Tracking and Metrics

### Migration Dashboard

| Component | Tier | Owner | Status | Start | End | Build Δ | Test Δ |
|-----------|------|-------|--------|-------|-----|---------|--------|
| rust_gtest_interop | 1 | TBD | 📋 Planned | - | - | - | - |
| build_tools | 1 | TBD | 📋 Planned | - | - | - | - |
| qr_code_generator | 1 | TBD | 📋 Planned | - | - | - | - |
| media_filters | 2 | TBD | ⏸️ Queued | - | - | - | - |
| bluetooth_parser | 2 | TBD | ⏸️ Queued | - | - | - | - |
| payment_validation | 2 | TBD | ⏸️ Queued | - | - | - | - |

Legend:
- 📋 Planned: Scheduled, ready to start
- 🚧 Active: Migration in progress
- ✅ Complete: Fully migrated
- ⏸️ Queued: Awaiting capacity
- ❌ Blocked: Waiting on dependency

### Success Metrics

**Per Component**:
- Build time (clean)
- Build time (incremental)
- Test execution time
- Binary size
- Migration duration (actual vs planned)

**Aggregate** (Phase 2 end):
- Components migrated: Target 10-15
- Average build improvement: Target >0% (no regression)
- Developer satisfaction: Target >75% positive
- Cargo adoption: Target 25-35% of Rust code

## Risk Mitigation

### Known Risks

**Risk 1: Build Time Regression**
- **Mitigation**: Benchmark early, optimize profiles
- **Fallback**: Keep GN build available

**Risk 2: Complex C++ Dependencies**
- **Mitigation**: Start simple, use patterns from Phase 1
- **Fallback**: Defer complex components to Tier 3

**Risk 3: Team Resistance**
- **Mitigation**: Training, documentation, support
- **Fallback**: Gradual rollout, champions program

**Risk 4: CI/CD Issues**
- **Mitigation**: Parallel CI, gradual rollout
- **Fallback**: Dedicated Cargo CI initially

## Resources

- [Cargo Adoption Plan](../../cargo_adoption_plan.md)
- [Cargo Basics](../training/cargo_basics.md)
- [Troubleshooting Guide](../training/troubleshooting.md)
- [Pilot Program](../training/pilot_program.md)
- [Phase 1.1 Progress](../../../prototypes/PHASE_1_1_PROGRESS.md)
- [Phase 1.2 Progress](../../../tools/cargo_migration/PHASE_1_2_PROGRESS.md)
- [Phase 1.3 Progress](../training/PHASE_1_3_PROGRESS.md)

## Next Steps

1. **Week 1-2**: Team kickoff and training
   - Review Phase 2 plan
   - Assign component owners
   - Schedule migrations

2. **Month 13-14**: First migrations (Tier 1)
   - Start with rust_gtest_interop
   - Document process
   - Refine tooling

3. **Month 15-18**: Complete Tier 1
   - Migrate remaining pure Rust components
   - Collect metrics
   - Prepare for Tier 2

4. **Month 19**: Mid-phase review
   - Assess progress
   - Adjust timeline if needed
   - Plan Tier 2 migrations

---

**Phase 2 Status**: Ready to begin  
**Next Update**: After first component migration
