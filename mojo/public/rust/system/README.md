# Mojo Rust System API - Tier 2 Component

**Status**: Phase 2 Tier 2 - Second Complex FFI Migration  
**Date Added to Workspace**: 2026-01-01

## Overview

The Mojo Rust System API provides safe Rust bindings to the Mojo C system API for inter-process communication. This is a **Tier 2 component**, meaning it has complex FFI integration and depends on Chromium's build infrastructure.

## Build Requirements

⚠️ **IMPORTANT**: This component requires a **hybrid build approach**. It cannot be built with Cargo alone.

### Why Hybrid Build?

This component depends on:
- GN-generated Rust bindings for the Mojo C API (`mojo_c_system_bindings`)
- The `chromium::import!` macro for importing GN-generated code
- Mojo C system library (`//mojo/public/c/system`)

These dependencies require GN/Ninja to generate bindings before Cargo can build the Rust component.

## Building

### Option 1: Using the Component Build Script (Recommended)

```bash
# From component directory
./mojo/public/rust/system/build_hybrid.sh

# Or from repository root
./tools/cargo_migration/hybrid_build.sh --component mojo-rust-system-api
```

The component script will:
1. Check for/generate GN build files
2. Build Mojo C bindings with Ninja
3. Build Rust component with Cargo

### Option 2: Manual Two-Step Build

```bash
# Step 1: Generate bindings and build C dependencies with GN/Ninja
gn gen out/Default
ninja -C out/Default mojo/public/rust:mojo_c_system_bindings

# Step 2: Build Rust component with Cargo
cargo build -p mojo-rust-system-api
```

### Option 3: GN Build Only

```bash
# Build everything with GN/Ninja (traditional approach)
ninja -C out/Default mojo/public/rust:mojo_rust_system_api
```

## CI/CD Integration

### Current Status

⚠️ **Note**: The Mojo Rust System API is currently **excluded from the pure Cargo CI workflow** because it requires GN-generated bindings. It is tested in the full Chromium/GN CI environment.

**Cargo CI** (`.github/workflows/cargo-ci.yml`):
- Runs on: Push to main/develop, Pull Requests
- Status: Excludes `mojo-rust-system-api` package
- Command: `cargo build --workspace --exclude ble-scan-parser --exclude mojo-rust-system-api`

**Chromium CI** (GN/Ninja based):
- Runs on: Full Chromium infrastructure
- Status: Includes all components
- Tests: `ninja -C out/Default rust_mojo_tests`

### Future Improvements

For full hybrid build CI support, we would need to:
1. Install GN and Ninja in CI environment
2. Generate build files before Cargo build
3. Build Mojo C API and bindings
4. Then run Cargo build

This is planned for future iterations as more Tier 2 components are added.

## Testing

### Cargo Tests

```bash
# After running GN build first
cargo test -p mojo-rust-system-api
```

### GN Tests

```bash
ninja -C out/Default rust_mojo_tests
out/Default/rust_mojo_tests
```

## Development Workflow

1. **First-time setup**: Run GN to generate bindings
   ```bash
   gn gen out/Default
   ninja -C out/Default mojo/public/rust:mojo_c_system_bindings
   ```

2. **Rust code changes**: You can use Cargo for faster iteration
   ```bash
   cargo build -p mojo-rust-system-api
   cargo test -p mojo-rust-system-api
   ```

3. **C API changes**: Regenerate bindings with GN/Ninja
   ```bash
   ninja -C out/Default mojo/public/rust:mojo_c_system_bindings
   ```

4. **Bindings changes**: Regenerate with GN/Ninja
   ```bash
   ninja -C out/Default
   ```

## Component Structure

```
system/
├── lib.rs                # Main Rust library (re-exports modules)
├── ffi.rs                # FFI layer (imports GN-generated bindings)
├── data_pipe.rs          # Data pipe abstractions
├── mojo_types.rs         # Mojo type definitions
├── raw_trap.rs           # Raw trap interface
├── safe_trap.rs          # Safe trap interface
├── build.rs              # Hybrid build script
├── Cargo.toml            # Cargo package manifest
└── README.md             # This file
```

## FFI Architecture

The component uses GN-generated bindings and Rust re-exports:

```
Mojo C System API (C library)
    ↓
GN-generated Rust bindings (via rust_bindgen)
    ↓
chromium::import! macro (ffi.rs)
    ↓
Safe Rust wrappers (data_pipe.rs, safe_trap.rs, etc.)
    ↓
Public API (lib.rs)
```

**Key types exposed**:
- `MojoHandle` - Opaque handle type
- `DataPipe` - Data pipe abstractions
- `Trap` - Event handling
- Various Mojo types and flags

## Dependencies

**External crates**:
- `bitflags` 2.0 - For flag types

**Chromium dependencies** (via GN):
- `mojo_c_system_bindings` - GN-generated Rust bindings
- `//mojo/public/c/system` - Mojo C API

## Known Limitations

1. **Requires GN-generated bindings**: Cannot build with pure Cargo
2. **chromium::import! macro**: Uses Chromium-specific build integration
3. **Dependency on Mojo C API**: Requires Mojo C system library

## Migration Notes

This component serves as the **second Tier 2 migration**, demonstrating the hybrid build pattern for Rust code that depends on GN-generated bindings.

**Key Learnings**:
- build.rs checks for GN output and provides helpful error messages
- Uses `chromium::import!` macro for GN integration
- Successfully added to workspace despite GN dependencies
- Hybrid build workflow validated

**Documentation**:
- See [TIER_2_PLANNING.md](/docs/rust/phase2/TIER_2_PLANNING.md) for overall strategy
- See [TIER_2_QUICK_REF.md](/docs/rust/phase2/TIER_2_QUICK_REF.md) for migration checklist
- See [MIGRATION_STATUS.md](/docs/rust/MIGRATION_STATUS.md) for project status

## Support

For questions or issues:
- **Slack**: #rust-migration, #mojo
- **Email**: rust-migration@chromium.org
- **Documentation**: https://chromium.org/rust/migration

---

**Last Updated**: 2026-01-01  
**Migration Status**: ✅ Added to Workspace  
**Next Steps**: Validate hybrid build in CI/CD
