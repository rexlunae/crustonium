# BLE Scan Parser - Tier 2 Component

**Status**: Phase 2 Tier 2 - First Complex C++ FFI Migration  
**Date Added to Workspace**: 2026-01-01

## Overview

The BLE (Bluetooth Low Energy) Scan Parser is a Rust component that parses Bluetooth advertising data. This is a **Tier 2 component**, meaning it has complex C++ FFI integration and depends on Chromium's build infrastructure.

## Build Requirements

⚠️ **IMPORTANT**: This component requires a **hybrid build approach**. It cannot be built with Cargo alone.

### Why Hybrid Build?

This component depends on:
- Chromium base library (`//base`)
- GN-generated header files (`base/debug/debugging_buildflags.h`, `build/build_config.h`)
- C++ types from `//device/bluetooth/public/cpp`

These dependencies require GN/Ninja to generate headers and build C++ libraries before Cargo can build the Rust component.

## Building

### Option 1: Using the Hybrid Build Script (Recommended)

```bash
# From repository root
./tools/cargo_migration/hybrid_build.sh

# Or explicitly specify hybrid mode
./tools/cargo_migration/hybrid_build.sh --system hybrid
```

This script will:
1. Run GN to generate build files and headers
2. Build C++ dependencies with Ninja
3. Build Rust components with Cargo

### Option 2: Manual Two-Step Build

```bash
# Step 1: Generate headers and build C++ dependencies with GN/Ninja
gn gen out/Default
ninja -C out/Default device/bluetooth

# Step 2: Build Rust component with Cargo
cargo build -p ble-scan-parser
```

### Option 3: GN Build Only

```bash
# Build everything with GN/Ninja (traditional approach)
ninja -C out/Default device/bluetooth/bluez/ble_scan_parser:lib
```

## CI/CD Integration

### Current Status

⚠️ **Note**: The BLE scan parser is currently **excluded from the pure Cargo CI workflow** because it requires GN-generated headers. It is tested in the full Chromium/GN CI environment.

**Cargo CI** (`.github/workflows/cargo-ci.yml`):
- Runs on: Push to main/develop, Pull Requests
- Status: Excludes `ble-scan-parser` package
- Command: `cargo build --workspace --exclude ble-scan-parser`

**Chromium CI** (GN/Ninja based):
- Runs on: Full Chromium infrastructure
- Status: Includes all components
- Tests: `ninja -C out/Default device_unittests`

### Future Improvements

For full hybrid build CI support, we would need to:
1. Install GN and Ninja in CI environment
2. Generate build files before Cargo build
3. Build C++ dependencies
4. Then run Cargo build

This is planned for future iterations as more Tier 2 components are added.

## Testing

### Cargo Tests

```bash
# After running GN build first
cargo test -p ble-scan-parser
```

### GN Tests

```bash
ninja -C out/Default device_unittests
out/Default/device_unittests --gtest_filter="BleScanParser*"
```

## Development Workflow

1. **First-time setup**: Run GN to generate headers
   ```bash
   gn gen out/Default
   ninja -C out/Default device/bluetooth
   ```

2. **Rust code changes**: You can use Cargo for faster iteration
   ```bash
   cargo build -p ble-scan-parser
   cargo test -p ble-scan-parser
   ```

3. **C++ wrapper changes**: Use Ninja to rebuild C++ components
   ```bash
   ninja -C out/Default device/bluetooth/bluez/ble_scan_parser:wrapper_functions
   ```

4. **Header changes**: Regenerate with GN/Ninja
   ```bash
   ninja -C out/Default
   ```

## Component Structure

```
ble_scan_parser/
├── lib.rs                    # Main Rust implementation
├── cxx.rs                    # CXX bridge definitions
├── build.rs                  # Hybrid build script
├── Cargo.toml                # Cargo package manifest
├── BUILD.gn                  # GN build configuration
├── wrapper_functions.{cc,h}  # C++ FFI wrapper
├── scan_record.{cc,h}        # Shared C++ types
├── ble_scan_parser.{cc,h}    # C++ entry point
├── ble_scan_parser_unittest.cc  # C++ unit tests
├── ble_scan_parser_fuzzer.cc    # Fuzzer
└── data/                     # Fuzzer test data
```

## FFI Architecture

The component uses the `cxx` crate for type-safe C++/Rust interop:

```
C++ Entry Point (ble_scan_parser.cc)
    ↓
C++ Wrapper (wrapper_functions.cc)
    ↓
CXX Bridge (cxx.rs)
    ↓
Rust Implementation (lib.rs)
```

**Key types exposed across FFI**:
- `ScanRecord` - Opaque C++ type (defined in scan_record.h)
- `UuidListBuilderForTest` - C++ helper for testing

**Functions exposed to C++**:
- `parse()` - Main parsing function
- `parse_service_uuids_for_test()` - Test helper
- `parse_uuid_for_test()` - Test helper

## Known Limitations

1. **Requires GN-generated headers**: Cannot build with pure Cargo
2. **Platform-specific**: Currently only built on ChromeOS (`assert(is_chromeos)` in BUILD.gn)
3. **Dependency on //base**: Requires Chromium base library

## Migration Notes

This component serves as the **first Tier 2 migration**, demonstrating the hybrid build pattern for complex C++ FFI components.

**Key Learnings**:
- build.rs checks for GN output and provides helpful error messages
- workspace.dependencies used for cxx and cxx-build versions
- Component successfully added to workspace despite GN dependencies
- Hybrid build workflow validated

**Documentation**:
- See [TIER_2_PLANNING.md](/docs/rust/phase2/TIER_2_PLANNING.md) for overall strategy
- See [TIER_2_QUICK_REF.md](/docs/rust/phase2/TIER_2_QUICK_REF.md) for migration checklist
- See [MIGRATION_STATUS.md](/docs/rust/MIGRATION_STATUS.md) for project status

## Support

For questions or issues:
- **Slack**: #rust-migration
- **Email**: rust-migration@chromium.org
- **Documentation**: https://chromium.org/rust/migration

---

**Last Updated**: 2026-01-01  
**Migration Status**: ✅ Added to Workspace  
**Next Steps**: Validate hybrid build in CI/CD
