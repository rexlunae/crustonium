# Cargo Migration Tooling

**Phase 1.2: Tooling Development**

This directory contains tools for migrating from GN/Ninja to Cargo build system.

## Tools

### 1. GN to Cargo Translator (`gn_to_cargo.py`)

Converts BUILD.gn files to Cargo.toml manifests.

**Features:**
- Parses `rust_static_library` targets
- Parses `cargo_crate` targets
- Generates Cargo.toml with proper dependencies
- Handles workspace dependencies
- Supports platform-specific configuration

**Usage:**

```bash
# List all Rust targets in a BUILD.gn file
python3 gn_to_cargo.py path/to/BUILD.gn --list

# Convert a specific target
python3 gn_to_cargo.py path/to/BUILD.gn --target my_target

# Generate Cargo.toml file
python3 gn_to_cargo.py path/to/BUILD.gn --target my_target -o Cargo.toml

# Convert with custom workspace root
python3 gn_to_cargo.py path/to/BUILD.gn --workspace /path/to/chromium
```

**Example:**

```bash
# Convert the QR code generator component
cd components/qr_code_generator
python3 ../../tools/cargo_migration/gn_to_cargo.py BUILD.gn --target qr_code_generator
```

**Limitations:**
- Current version uses regex-based parsing (simplified)
- Production version should use proper GN AST parser
- Cross-language dependencies need manual review
- Platform-specific configs may need adjustment

### 2. Hybrid Build Wrapper (`hybrid_build.sh`)

Unified build script supporting GN/Ninja, Cargo, or both.

**Features:**
- Build with GN/Ninja (existing system)
- Build with Cargo (Rust-only)
- Hybrid builds (Cargo + GN/Ninja)
- Automatic tool detection
- Build caching support
- Test and benchmark integration

**Usage:**

```bash
# Build with hybrid system (default)
./hybrid_build.sh

# Build with Cargo only
./hybrid_build.sh --system cargo

# Build with GN only
./hybrid_build.sh --system gn chrome

# Clean build
./hybrid_build.sh --clean

# Build and test
./hybrid_build.sh --test

# Run benchmarks
./hybrid_build.sh --bench

# Custom configuration
BUILD_CONFIG=Release ./hybrid_build.sh --system hybrid
```

**Environment Variables:**
- `BUILD_SYSTEM` - Override build system (gn|cargo|hybrid)
- `BUILD_CONFIG` - GN build configuration (Default, Release, etc.)
- `CARGO_PROFILE` - Cargo build profile (dev, release, production)

**Examples:**

```bash
# Development build with Cargo
CARGO_PROFILE=dev ./hybrid_build.sh --system cargo

# Release build with hybrid system
BUILD_CONFIG=Release CARGO_PROFILE=release ./hybrid_build.sh

# Parallel build with 8 jobs
./hybrid_build.sh --jobs 8

# Quick check without building
./hybrid_build.sh --check
```

## CI/CD Integration

### GitHub Actions

The `.github/workflows/cargo-ci.yml` workflow provides:

- **Multi-platform builds**: Linux, Windows, macOS
- **Automated testing**: Unit tests and doc tests
- **Security scanning**: cargo-audit for vulnerabilities
- **Dependency checking**: cargo-deny for license compliance
- **Code coverage**: Optional coverage reporting
- **Build caching**: Cargo registry and build artifacts

**Workflow triggers:**
- Push to main/develop branches
- Pull requests
- Manual workflow dispatch

**Jobs:**
1. `check` - Quick formatting and lint checks
2. `build` - Multi-platform compilation
3. `test` - Cross-platform testing
4. `audit` - Security vulnerability scan
5. `deny` - Dependency policy enforcement
6. `coverage` - Code coverage (main branch only)

### Local CI Simulation

Test CI workflow locally before pushing:

```bash
# Install act (GitHub Actions local runner)
# brew install act  # macOS
# sudo apt install act  # Linux

# Run the workflow
act -j build
act -j test
```

## Migration Workflow

### Step 1: Analyze Component

```bash
# Find Rust targets
python3 tools/cargo_migration/gn_to_cargo.py path/to/BUILD.gn --list
```

### Step 2: Generate Cargo.toml

```bash
# Generate Cargo.toml from BUILD.gn
python3 tools/cargo_migration/gn_to_cargo.py path/to/BUILD.gn \
  --target your_target \
  --output path/to/Cargo.toml
```

### Step 3: Review and Adjust

- Review generated Cargo.toml
- Adjust dependencies if needed
- Add to workspace members in root Cargo.toml
- Test build with Cargo

### Step 4: Test Hybrid Build

```bash
# Build with hybrid system
./tools/cargo_migration/hybrid_build.sh --system hybrid --test
```

### Step 5: Validate

```bash
# Run all checks
cargo check --workspace
cargo test --workspace
cargo clippy --workspace

# Compare outputs
./tools/cargo_migration/hybrid_build.sh --system gn
./tools/cargo_migration/hybrid_build.sh --system cargo
# Verify identical functionality
```

## Development

### Adding Features

**To add GN target type support:**
1. Add parser method to `GNParser` class
2. Add generator method to `CargoGenerator` class
3. Update conversion logic in `generate_from_gn()`
4. Add test cases

**To extend hybrid build script:**
1. Add new function for feature
2. Add command-line argument
3. Integrate into `main()` function
4. Update help text

### Testing Tools

```bash
# Test GN parser
python3 -m pytest tools/cargo_migration/tests/  # (when tests added)

# Test hybrid build script
./tools/cargo_migration/hybrid_build.sh --help
./tools/cargo_migration/hybrid_build.sh --check

# Manual testing
cd prototypes/cargo_cpp_integration
python3 ../../tools/cargo_migration/gn_to_cargo.py BUILD.gn --list
```

## Troubleshooting

### GN Parser Issues

**Problem**: Can't find targets
```bash
# Solution: Check file path and format
python3 gn_to_cargo.py BUILD.gn --list
```

**Problem**: Invalid Cargo.toml generated
```bash
# Solution: Review generated file, adjust manually, report issue
cargo check  # Will show specific errors
```

### Hybrid Build Issues

**Problem**: Tools not found
```bash
# Solution: Install required tools
# GN: Download from https://gn.googlesource.com/gn/
# Ninja: apt install ninja-build / brew install ninja
# Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Problem**: Build failures
```bash
# Solution: Check individual build systems
./hybrid_build.sh --system gn   # Test GN build
./hybrid_build.sh --system cargo # Test Cargo build
# Fix issues in each before using hybrid
```

### CI Issues

**Problem**: Workflow fails
```bash
# Solution: Check logs in GitHub Actions
# Run locally with act to debug:
act -j build --verbose
```

## Future Enhancements

### Planned Features

**Phase 1.3** (Documentation and Training):
- [ ] Interactive migration wizard
- [ ] Automated dependency resolution
- [ ] Build output comparison tool
- [ ] Performance profiling integration

**Phase 2** (Incremental Migration):
- [ ] Batch conversion scripts
- [ ] Workspace graph visualization
- [ ] Build time tracking and comparison
- [ ] Automated rollback capability

**Long-term**:
- [ ] Full GN AST parser
- [ ] CMake integration for complex C++
- [ ] Binary compatibility checker
- [ ] Migration progress dashboard

## References

- [Cargo Adoption Plan](../../docs/cargo_adoption_plan.md)
- [Hybrid Build Setup Guide](../../docs/hybrid_build_setup.md)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [GN Reference](https://gn.googlesource.com/gn/+/main/docs/reference.md)

## Support

For issues or questions:
- File an issue with `build-system` label
- Slack: `#cargo-migration`
- Email: `build-dev@chromium.org`
- Documentation: `/docs/cargo_adoption_plan.md`

---

**Phase 1.2 Status**: Implementation complete ✅
**Next Phase**: 1.3 - Documentation and Training
