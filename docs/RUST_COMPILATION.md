# Rust Compilation Implementation Guide

This document describes the implementation of Rust compilation support in Crustonium using Cargo alongside the existing GN/Ninja build system.

## Overview

Crustonium now supports building Rust components using Cargo, the standard Rust build tool and package manager. This implementation enables:

- Compiling Rust code with standard tooling
- Managing Rust dependencies through Cargo
- Building C++/Rust FFI bridges using the `cxx` crate
- Maintaining compatibility with the existing GN build system

## Current Status

### Successfully Migrated Components

1. **QR Code Generator** (`components/qr_code_generator`)
   - Rust FFI glue for QR code generation
   - Uses `cxx` for type-safe C++ interop
   - Depends on `qr_code` crate from crates.io
   - Successfully builds and integrates with C++ components

### Workspace Structure

```
crustonium/
├── Cargo.toml                    # Workspace root configuration
├── .cargo/
│   └── config.toml              # Cargo configuration
├── prototypes/
│   ├── cargo_cpp_integration/   # C++ integration prototype
│   └── workspace_structure_test/  # Workspace structure tests
└── components/
    └── qr_code_generator/
        ├── Cargo.toml           # Component-specific configuration
        ├── build.rs             # Build script for cxx bridge
        └── qr_code_generator_ffi_glue.rs  # Rust FFI implementation
```

## Building Rust Components

### Building the Entire Workspace

```bash
# Build all Rust components
cargo build --workspace

# Build in release mode
cargo build --workspace --release

# Build a specific component
cargo build -p qr-code-generator-ffi
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific component
cargo test -p qr-code-generator-ffi
```

### Running Benchmarks

```bash
# Run benchmarks for prototypes
cargo bench -p cargo-cpp-integration-prototype
```

## Adding New Rust Components

To add a new Rust component to the Cargo build system:

### 1. Create a Cargo.toml

In your component directory, create a `Cargo.toml` file:

```toml
[package]
name = "your-component-name"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "your_component"
crate-type = ["staticlib", "rlib"]
path = "your_file.rs"

[dependencies]
# FFI support (if needed)
cxx = { workspace = true }

# Add other dependencies
serde = { workspace = true }

[build-dependencies]
cxx-build = { workspace = true }
```

### 2. Create a build.rs (if using cxx)

If your component uses C++/Rust FFI via cxx:

```rust
fn main() {
    // Get the workspace root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    // Adjust this path based on your component location
    let workspace_root = manifest_dir
        .rsplit_once("/components/your_component")
        .expect("Expected to be in components/your_component")
        .0;

    // Build the cxx bridge
    cxx_build::bridge("your_file.rs")
        .flag_if_supported("-std=c++17")
        .include(workspace_root)  // Include workspace root for headers
        .compile("your_component");

    println!("cargo:rerun-if-changed=your_file.rs");
}
```

### 3. Update Workspace Cargo.toml

Add your component to the workspace members:

```toml
[workspace]
members = [
    # ... existing members ...
    "components/your_component",
]
```

If you're using new dependencies, add them to workspace.dependencies:

```toml
[workspace.dependencies]
your-dependency = "version"
```

### 4. Test the Build

```bash
# Build your component
cargo build -p your-component-name

# Run tests
cargo test -p your-component-name
```

## Workspace Configuration

### Build Profiles

The workspace defines several build profiles optimized for different use cases:

- **dev**: Fast compilation for development (opt-level = 0)
- **release**: Optimized release builds (opt-level = 3, thin LTO)
- **production**: Maximum optimization (opt-level = 3, fat LTO, single codegen unit)
- **bench**: Performance testing (inherits release, with debug symbols)

### Cargo Configuration

The `.cargo/config.toml` file configures:

- Linker settings (uses `lld` for faster linking)
- Platform-specific build flags
- Convenient cargo aliases

## FFI Integration with C++

Components that bridge Rust and C++ use the `cxx` crate for type-safe interop:

1. Define the FFI boundary in your Rust file using `#[cxx::bridge]`
2. Include necessary C++ headers in the bridge definition
3. Implement Rust functions that C++ can call
4. Use the build script to compile the generated C++ code

Example:

```rust
#[cxx::bridge(namespace = "your_namespace")]
mod ffi {
    extern "C++" {
        include!("components/your_component/header.h");
        type YourCppType;
    }

    extern "Rust" {
        fn your_rust_function(param: &str) -> bool;
    }
}

pub fn your_rust_function(param: &str) -> bool {
    // Implementation
    true
}
```

## Dependency Management

### Using Crates.io Dependencies

Dependencies from crates.io should be added to `[workspace.dependencies]` for version consistency:

```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
your-crate = "x.y"
```

Components then reference workspace dependencies:

```toml
[dependencies]
serde = { workspace = true }
your-crate = { workspace = true }
```

### Third-Party Dependencies

For dependencies that need to be vendored or come from Chromium's third_party directory, additional configuration may be needed. Currently, the workspace uses standard crates.io dependencies.

## Hybrid Build System

The Cargo build system operates alongside the existing GN/Ninja build system:

- **GN/Ninja**: Still builds the entire Chromium/Crustonium project including C++ code
- **Cargo**: Builds Rust components independently for development and testing
- Both systems can compile the same Rust code

This allows developers to:
- Use standard Rust tooling for Rust development
- Benefit from Cargo's dependency management
- Integrate with existing Chromium build infrastructure

## Development Workflow

### Recommended Workflow for Rust Development

1. Make changes to Rust code
2. Build and test using Cargo:
   ```bash
   cargo build -p your-component
   cargo test -p your-component
   ```
3. Once changes are working, ensure they still build with GN:
   ```bash
   # In the Chromium build directory
   ninja your_target
   ```

### IDE Integration

Standard Rust IDE tools work with this setup:

- **rust-analyzer**: Automatically detects the workspace and provides code intelligence
- **clippy**: `cargo clippy --workspace` for linting
- **rustfmt**: `cargo fmt --all` for code formatting

## Troubleshooting

### Build Failures

1. **Missing headers**: Ensure your `build.rs` includes the workspace root:
   ```rust
   .include(workspace_root)
   ```

2. **Dependency issues**: Make sure all dependencies are declared in both the workspace and component `Cargo.toml`

3. **cxx bridge errors**: Check that all C++ headers referenced in `extern "C++"` blocks are available

### Testing

Run `cargo test --workspace --verbose` for detailed test output.

## Future Work

### Components to Migrate

- `components/user_data_importer/utility/parsing_ffi` (requires custom dependencies)
- `media/filters` (media processing components)
- `testing/rust_gtest_interop` (test infrastructure)
- Additional Rust components as they are developed

### Improvements

- Automated GN-to-Cargo translation tools
- Unified build caching between GN and Cargo
- CI/CD integration for Cargo builds
- Expanded component migration

## References

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [cxx Documentation](https://cxx.rs/)
- [Cargo Adoption Plan](../docs/cargo_adoption_plan.md)
- [Rust Adoption Plan](../docs/rust_adoption_plan.md)
- [Hybrid Build Setup Guide](../docs/hybrid_build_setup.md)

## Contributing

When adding new Rust components:

1. Follow the component addition guide above
2. Ensure tests pass: `cargo test --workspace`
3. Document any new dependencies or special requirements
4. Update this README if needed

For questions or issues, consult the Rust adoption team or file an issue in the repository.
