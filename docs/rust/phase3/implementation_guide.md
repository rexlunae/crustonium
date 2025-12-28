# Phase 3: Comprehensive Migration - Implementation Guide

**Status**: Framework Ready  
**Duration**: Months 31-48 (18 months)  
**Goal**: Majority of builds use Cargo, GN/Ninja in maintenance mode

[TOC]

## Overview

Phase 3 represents the comprehensive migration where Cargo becomes the primary build system for the majority of the codebase. By this phase, teams have gained experience through Phase 2 migrations, patterns are well-established, and the focus shifts to migrating larger, more complex subsystems while maintaining GN/Ninja support for legacy components.

### Phase 3 Objectives

1. **Scale Migration**: Migrate major subsystems and core components (30-50 additional components)
2. **Establish Cargo as Primary**: Make Cargo the default build path for new development
3. **Standardize C++ Integration**: Solidify patterns for building complex C++ dependencies
4. **Optimize Build Performance**: Ensure Cargo builds match or exceed GN/Ninja performance
5. **Enhance Developer Experience**: Full IDE integration and standard Rust tooling adoption
6. **Maintain Compatibility**: Keep GN/Ninja functional for remaining legacy components

### Phase 3 Sub-Phases

**3.1 Core Component Migration** (Months 31-40)
- Major subsystems (network, storage, platform abstraction)
- Complex components with significant C++ integration
- Critical path components

**3.2 Developer Experience Enhancement** (Months 31-36)
- IDE integration finalization
- Standard Rust tooling adoption
- Build optimization and distributed builds

**3.3 Infrastructure Transformation** (Months 37-44)
- CI/CD migration to Cargo-primary
- Release process updates
- Binary packaging and distribution

**3.4 Consolidation** (Months 45-48)
- Remaining component migrations
- GN/Ninja deprecation planning
- Documentation and training updates

## Success Metrics

By end of Phase 3 (Month 48):
- **Component Coverage**: 60-75% of components use Cargo as primary build
- **Build Performance**: Cargo builds ≤ 105% of GN/Ninja baseline (clean + incremental)
- **Developer Adoption**: >80% of developers use Cargo for daily work
- **CI/CD**: Primary CI path uses Cargo for all migrated components
- **Code Coverage**: 65-80% of Rust code uses Cargo build system
- **New Development**: 95%+ of new Rust components use Cargo from start

## 3.1 Core Component Migration (Months 31-40)

### Migration Categories

#### Category A: Network Stack Components

**Target Components** (10-15 components):
1. **HTTP/HTTPS Handlers**
   - `net/http/` - HTTP protocol implementation
   - `net/quic/` - QUIC protocol support
   - Risk: High (performance critical)
   - Complexity: High (C++ integration heavy)
   
2. **DNS and Socket Management**
   - `net/dns/` - DNS resolution
   - `net/socket/` - Socket abstractions
   - Risk: Medium-High
   - Complexity: Medium
   
3. **Certificate and Security**
   - `net/cert/` - Certificate validation
   - `net/ssl/` - SSL/TLS handling
   - Risk: High (security critical)
   - Complexity: High

**Migration Approach**:
```toml
# Cargo.toml for network components
[package]
name = "chromium-net-http"
version = "1.0.0"
edition.workspace = true

[dependencies]
# Rust networking crates
tokio = { version = "1.35", features = ["net", "rt-multi-thread"] }
hyper = "1.0"
rustls = "0.22"

# Internal dependencies
chromium-net-base.workspace = true
chromium-ssl.workspace = true

# C++ integration
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
cc = "1.0"

[lib]
crate-type = ["staticlib", "rlib"]
```

**build.rs Pattern**:
```rust
fn main() {
    // Compile C++ implementation files
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .files(&[
            "src/legacy/http_parser.cc",
            "src/legacy/connection_pool.cc",
        ])
        .include("src/legacy")
        .compile("net_http_legacy");
    
    // cxx bridge for FFI
    cxx_build::bridge("src/ffi.rs")
        .file("src/bridge_impl.cc")
        .flag("-std=c++17")
        .include("src")
        .compile("net_http_bridge");
    
    // Link dependencies
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    
    println!("cargo:rerun-if-changed=src/legacy/");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
```

#### Category B: Storage and Persistence

**Target Components** (8-12 components):
1. **Database Abstractions**
   - `storage/browser/database/` - Database layer
   - `sql/` - SQL database support
   - Risk: Medium
   - Complexity: Medium-High
   
2. **File System**
   - `storage/browser/file_system/` - File system APIs
   - Risk: Medium
   - Complexity: Medium
   
3. **Cache and IndexedDB**
   - `storage/browser/cache_storage/` - Cache API
   - `content/browser/indexed_db/` - IndexedDB implementation
   - Risk: High (data integrity critical)
   - Complexity: High

**Migration Timeline**: 10-12 weeks per major component

#### Category C: Platform Abstraction Layers

**Target Components** (6-10 components):
1. **Base Abstractions**
   - `base/` (select components) - Core abstractions
   - Risk: Very High (affects everything)
   - Complexity: Very High
   - Strategy: Incremental, sub-component by sub-component
   
2. **Threading and Tasks**
   - `base/task/` - Task scheduling
   - `base/threading/` - Thread management
   - Risk: High
   - Complexity: High
   
3. **Memory Management**
   - `base/memory/` - Memory utilities
   - Risk: High
   - Complexity: Medium-High

**Special Considerations**:
- Platform abstraction components require extreme care
- Migrate in small increments with extensive testing
- Maintain parallel GN builds for extended period
- Performance benchmarking at each step

#### Category D: Resource Management

**Target Components** (5-8 components):
1. **Resource Loading**
   - `content/browser/loader/` - Resource loader
   - Risk: Medium-High
   - Complexity: High
   
2. **Memory and Process Management**
   - `content/browser/memory/` - Memory coordination
   - Risk: High
   - Complexity: Medium

### 10-Week Migration Process (Complex Components)

**Week 1-2: Deep Analysis and Planning**
- Architecture review and dependency mapping
- Identify all C++ integration points
- Performance baseline measurements
- Risk assessment and mitigation planning
- Team alignment and resource allocation

**Week 3-4: Cargo Structure Setup**
- Create comprehensive Cargo.toml
- Design build.rs for C++ integration
- Set up FFI boundaries with cxx
- Configure workspace integration
- Initial compilation attempt

**Week 5-6: Build System Integration**
- Resolve compilation errors
- Configure platform-specific builds
- Integrate with CMake if needed
- Validate build outputs match GN
- Performance comparison (build times)

**Week 7-8: Testing and Validation**
- Port all tests to Cargo
- Run comprehensive test suites
- Performance benchmarking (runtime)
- Memory profiling
- Security validation

**Week 9: CI/CD and Documentation**
- Update CI/CD pipelines
- Hybrid BUILD.gn configuration
- Update developer documentation
- Create migration report
- Knowledge sharing session

**Week 10: Stabilization and Handoff**
- Address feedback and issues
- Final performance validation
- Team training and handoff
- Post-migration monitoring setup
- Retrospective and lessons learned

## 3.2 Developer Experience Enhancement (Months 31-36)

### IDE Integration

#### rust-analyzer Configuration

**Workspace `.vscode/settings.json`**:
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
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

### Standard Rust Tooling Adoption

#### cargo clippy - Linting

**Configuration** (`.clippy.toml`):
```toml
# Clippy configuration
msrv = "1.75.0"  # Minimum Supported Rust Version

# Allowed lints (project-specific exceptions)
allow = [
    "too_many_arguments",
    "type_complexity",
]

# Warnings as errors for CI
warn = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::cargo",
]

# Explicitly denied lints
deny = [
    "clippy::unwrap_used",
    "clippy::expect_used",
    "clippy::panic",
]
```

#### cargo fmt - Formatting

**Configuration** (`rustfmt.toml`):
```toml
# Rust formatting configuration
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
wrap_comments = true
format_code_in_doc_comments = true
normalize_comments = true
format_strings = true
format_macro_bodies = true
```

#### cargo-deny - Dependency Auditing

**Configuration** (`deny.toml`):
```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
]
deny = [
    "GPL-3.0",
    "AGPL-3.0",
]

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

### Build Optimization

#### Distributed Builds with sccache

**Setup**:
```toml
# .cargo/config.toml
[build]
rustc-wrapper = "/usr/local/bin/sccache"

[env]
SCCACHE_DIR = "/var/cache/sccache"
SCCACHE_CACHE_SIZE = "50G"
```

## 3.3 Infrastructure Transformation (Months 37-44)

### CI/CD Migration to Cargo-Primary

**Primary Build Path** (Cargo):
```yaml
# .github/workflows/main-build.yml
name: Main Build (Cargo)

on:
  push:
    branches: [main, release/*]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  build:
    name: Build (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, nightly]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy
      
      - name: Build workspace
        run: cargo build --workspace --all-features --all-targets
      
      - name: Run tests
        run: cargo test --workspace --all-features
      
      - name: Run clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
        if: matrix.rust == 'stable'
```

### Release Process Updates

#### Binary Packaging from Cargo

**Release Build Configuration**:
```toml
# Cargo.toml workspace
[profile.release-production]
inherits = "release"
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "symbols"
panic = "abort"
```

## 3.4 Consolidation (Months 45-48)

### GN/Ninja Deprecation Planning

#### Deprecation Timeline

**Month 45**: Deprecation announcement
- Communicate GN/Ninja sunset timeline
- Provide migration resources
- Establish exception process

**Month 46-47**: Transition period
- No new GN-only components
- Hybrid support maintained
- Document exceptions

**Month 48**: GN/Ninja archive preparation
- Final migration of portable components
- Documentation of exceptions
- Archive build files for reference

## Risk Mitigation

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Performance regression in migrated components | High | Medium | Continuous benchmarking, rollback plan |
| C++ integration complexity blocks migration | High | Medium | CMake integration, incremental approach |
| Build system compatibility issues | Medium | Low | Extensive testing, hybrid support |
| Platform-specific build failures | Medium | Medium | Cross-platform CI, platform specialists |

### Organizational Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Team resistance to change | Medium | Medium | Change management, training, champions |
| Resource constraints | High | Medium | Prioritization, external help, tooling |
| Knowledge gaps | Medium | Low | Documentation, training, pair programming |

## Success Criteria

### Quantitative Metrics

- **Migration Coverage**: ≥60% components using Cargo
- **Build Performance**: ≤105% of GN/Ninja baseline
- **Test Pass Rate**: 100% parity with GN builds
- **Binary Size**: ≤102% of GN builds
- **CI/CD Reliability**: ≥99.5% success rate
- **Developer Satisfaction**: ≥4.2/5.0 average rating

### Qualitative Goals

- Cargo is the default choice for new development
- Developers prefer Cargo workflow over GN
- Clear path forward for remaining C++ components
- Standard Rust tooling fully integrated
- Documentation comprehensive and current

## Next Steps

### Immediate Actions (Month 31)

1. **Kickoff Meeting**
   - Review Phase 3 goals and timeline
   - Assign component owners
   - Establish working groups

2. **Infrastructure Setup**
   - Finalize CI/CD Cargo-primary configuration
   - Set up performance monitoring
   - Configure distributed build cache

3. **First Migration Wave**
   - Begin network stack component migrations
   - Establish complex C++ integration patterns
   - Track metrics and learnings

### Month 32-40 Focus

- Execute core component migrations
- Optimize build performance continuously
- Expand IDE integration support
- Regular retrospectives and process improvements

### Month 41-48 Focus

- Complete remaining migrations
- Finalize GN/Ninja deprecation
- Comprehensive documentation update
- Prepare for Phase 4 (Cargo-only)
