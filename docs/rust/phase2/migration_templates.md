# Phase 2: Migration Templates and Examples

**Phase 2: Incremental Migration**

Ready-to-use templates and real examples for component migration.

[TOC]

## Component Cargo.toml Templates

### Template 1: Pure Rust Component

```toml
# crates/COMPONENT_NAME/Cargo.toml
# Phase 2: Tier 1 Migration

[package]
name = "chromium-COMPONENT_NAME"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true

description = "Brief description of component functionality"

[dependencies]
# Workspace dependencies (preferred)
serde.workspace = true
thiserror.workspace = true
log.workspace = true

# External dependencies
external_crate = "1.0"

# Optional features
[features]
default = ["std"]
std = []
experimental = []

[lib]
name = "component_name"
path = "src/lib.rs"
crate-type = ["staticlib", "rlib"]

[dev-dependencies]
criterion.workspace = true

[[bench]]
name = "benchmarks"
harness = false
```

### Template 2: Rust Component with C++ FFI (cxx)

```toml
# crates/COMPONENT_NAME/Cargo.toml
# Phase 2: Tier 2 Migration (Rust + C++ FFI)

[package]
name = "chromium-COMPONENT_NAME"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true

description = "Component with C++ integration via cxx"

[dependencies]
# FFI dependencies
cxx.workspace = true

# Other workspace dependencies
serde.workspace = true
log.workspace = true

[build-dependencies]
cxx-build.workspace = true

[lib]
name = "component_name"
path = "src/lib.rs"
crate-type = ["staticlib", "rlib"]
```

### Template 3: Component with Complex C++ Build

```toml
# crates/COMPONENT_NAME/Cargo.toml
# Phase 2: Tier 2 Migration (CMake build)

[package]
name = "chromium-COMPONENT_NAME"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true

description = "Component with complex C++ dependencies"

[dependencies]
cxx.workspace = true

[build-dependencies]
cxx-build.workspace = true
cmake.workspace = true
cc.workspace = true

[lib]
name = "component_name"
path = "src/lib.rs"
crate-type = ["staticlib", "rlib"]
```

## build.rs Templates

### Template 1: Simple C++ Compilation (cc crate)

```rust
// crates/COMPONENT_NAME/build.rs
// Simple C++ file compilation

use std::env;

fn main() {
    // Compile C++ files
    cc::Build::new()
        .cpp(true)
        .file("cpp/implementation.cc")
        .file("cpp/utilities.cc")
        .flag("-std=c++17")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .warnings(true)
        .compile("component_cpp");
    
    // Link against C++ standard library
    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
    
    // Rerun if C++ files change
    println!("cargo:rerun-if-changed=cpp/");
}
```

### Template 2: cxx Bridge Compilation

```rust
// crates/COMPONENT_NAME/build.rs
// cxx bridge compilation

fn main() {
    // Build cxx bridge
    cxx_build::bridge("src/ffi.rs")
        .file("src/bridge_impl.cc")
        .flag("-std=c++17")
        .flag_if_supported("-Wall")
        .include("src")
        .include("cpp")
        .compile("component_bridge");
    
    // Platform-specific linking
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++");
    
    // Rebuild if files change
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/bridge_impl.cc");
    println!("cargo:rerun-if-changed=src/bridge_impl.h");
}
```

### Template 3: CMake Integration

```rust
// crates/COMPONENT_NAME/build.rs
// CMake-based C++ build

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("cpp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .build();
    
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=component_cpp");
    
    // Platform-specific linking
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
    
    // Rerun if CMakeLists.txt or sources change
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/src/");
}
```

### Template 4: Combined cc + cxx

```rust
// crates/COMPONENT_NAME/build.rs
// Combined approach: cc for legacy C++, cxx for new FFI

fn main() {
    // Build legacy C++ code with cc
    cc::Build::new()
        .cpp(true)
        .file("cpp/legacy.cc")
        .flag("-std=c++17")
        .compile("legacy");
    
    // Build cxx bridge for new FFI
    cxx_build::bridge("src/ffi.rs")
        .file("src/bridge.cc")
        .flag("-std=c++17")
        .include("cpp")
        .compile("bridge");
    
    // Link everything
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=cpp/");
}
```

## FFI Examples

### Example 1: Basic cxx Bridge

**src/ffi.rs**:
```rust
#[cxx::bridge(namespace = "my_component")]
mod ffi {
    // C++ types we'll use
    extern "C++" {
        include!("my_component/wrapper.h");
        
        // Opaque C++ type
        type CppWidget;
        
        // C++ functions
        fn create_widget() -> UniquePtr<CppWidget>;
        fn process_data(widget: &CppWidget, data: &[u8]) -> Vec<u8>;
    }
    
    // Rust functions exposed to C++
    extern "Rust" {
        fn rust_callback(event: &str) -> bool;
    }
}

// Rust implementation
pub fn rust_callback(event: &str) -> bool {
    println!("Event: {}", event);
    true
}
```

**src/bridge.cc**:
```cpp
#include "my_component/wrapper.h"
#include "crates/my_component/src/ffi.rs.h"

namespace my_component {

std::unique_ptr<CppWidget> create_widget() {
    return std::make_unique<CppWidget>();
}

rust::Vec<uint8_t> process_data(const CppWidget& widget,
                                  rust::Slice<const uint8_t> data) {
    rust::Vec<uint8_t> result;
    // Process data...
    return result;
}

}  // namespace my_component
```

### Example 2: Shared Types

**src/ffi.rs**:
```rust
#[cxx::bridge(namespace = "shared")]
mod ffi {
    // Shared struct (Rust side)
    #[derive(Debug, Clone)]
    struct Config {
        enabled: bool,
        timeout_ms: i32,
        name: String,
    }
    
    extern "C++" {
        include!("shared/processor.h");
        
        type Processor;
        
        fn create_processor(config: &Config) -> UniquePtr<Processor>;
        fn process(self: &Processor, input: &[u8]) -> Vec<u8>;
    }
}
```

**shared/processor.h**:
```cpp
#pragma once
#include "rust/cxx.h"
#include "crates/shared/src/ffi.rs.h"

namespace shared {

class Processor {
 public:
  explicit Processor(const Config& config);
  rust::Vec<uint8_t> process(rust::Slice<const uint8_t> input);
  
 private:
  Config config_;
};

}  // namespace shared
```

### Example 3: Error Handling

**src/ffi.rs**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
}

#[cxx::bridge(namespace = "processor")]
mod ffi {
    extern "Rust" {
        type ProcessError;
        
        fn process_safe(data: &[u8]) -> Result<Vec<u8>>;
    }
}

pub fn process_safe(data: &[u8]) -> Result<Vec<u8>, ProcessError> {
    if data.is_empty() {
        return Err(ProcessError::InvalidInput("Empty input".to_string()));
    }
    
    // Process...
    Ok(vec![])
}
```

## BUILD.gn Integration Templates

### Template 1: Basic Cargo Integration

```python
# components/COMPONENT_NAME/BUILD.gn

import("//build/rust/cargo_crate.gni")

cargo_crate("component_name") {
  crate_root = "../../crates/component_name/Cargo.toml"
  output_name = "chromium_component_name"
  
  # Cargo features to enable
  features = []
  
  # GN dependencies (if any)
  deps = []
}

# Use the Cargo-built component
group("component_name_default") {
  public_deps = [ ":component_name" ]
}
```

### Template 2: Hybrid Build (Cargo + Legacy)

```python
# components/COMPONENT_NAME/BUILD.gn

import("//build/rust/cargo_crate.gni")
import("//build/rust/rust_static_library.gni")

# New Cargo-based build (preferred)
cargo_crate("component_name_cargo") {
  crate_root = "../../crates/component_name/Cargo.toml"
  output_name = "chromium_component_name"
  features = []
}

# Legacy GN build (fallback)
rust_static_library("component_name_legacy") {
  crate_root = "src/lib.rs"
  sources = [
    "src/lib.rs",
    "src/module.rs",
  ]
  
  deps = [
    "//third_party/rust:some_crate",
  ]
  
  # Mark as deprecated
  visibility = [ ":*" ]
}

# Choose which build to use
if (enable_cargo_build) {
  group("component_name") {
    public_deps = [ ":component_name_cargo" ]
  }
} else {
  group("component_name") {
    public_deps = [ ":component_name_legacy" ]
  }
}
```

### Template 3: Component with Tests

```python
# components/COMPONENT_NAME/BUILD.gn

import("//build/rust/cargo_crate.gni")
import("//testing/test.gni")

cargo_crate("component_name") {
  crate_root = "../../crates/component_name/Cargo.toml"
  output_name = "chromium_component_name"
}

# Cargo tests (preferred)
test("component_name_cargo_tests") {
  sources = []
  
  # Run via cargo test
  script = "//tools/cargo_migration/run_cargo_test.py"
  args = [
    "--package",
    "chromium-component-name",
  ]
}

# GN tests (if needed)
test("component_name_tests") {
  sources = [
    "test/test_file.cc",
  ]
  
  deps = [
    ":component_name",
    "//testing/gtest",
  ]
}
```

## Migration Script Templates

### Template: Migration Preparation Script

```bash
#!/bin/bash
# scripts/prepare_migration.sh
# Prepares a component for Cargo migration

set -e

COMPONENT=$1
if [ -z "$COMPONENT" ]; then
    echo "Usage: $0 <component_name>"
    exit 1
fi

echo "=== Preparing $COMPONENT for Cargo migration ==="

# 1. Generate Cargo.toml
echo "Generating Cargo.toml..."
python3 tools/cargo_migration/gn_to_cargo.py \
    components/$COMPONENT/BUILD.gn \
    --target $COMPONENT \
    --output /tmp/${COMPONENT}_Cargo.toml

# 2. Create crate structure
echo "Creating crate structure..."
mkdir -p crates/$COMPONENT/src
mkdir -p crates/$COMPONENT/cpp

# 3. Copy Rust sources
echo "Copying Rust sources..."
find components/$COMPONENT -name "*.rs" -exec cp {} crates/$COMPONENT/src/ \;

# 4. Copy Cargo.toml
cp /tmp/${COMPONENT}_Cargo.toml crates/$COMPONENT/Cargo.toml

# 5. Create README
cat > crates/$COMPONENT/README.md <<EOF
# $COMPONENT

Migrated to Cargo as part of Phase 2.

## Building

\`\`\`bash
cargo build -p chromium-$COMPONENT
\`\`\`

## Testing

\`\`\`bash
cargo test -p chromium-$COMPONENT
\`\`\`

## Original Location

- Original: \`components/$COMPONENT/\`
- Migration date: $(date +%Y-%m-%d)
EOF

echo "=== Preparation complete ==="
echo "Next steps:"
echo "1. Review crates/$COMPONENT/Cargo.toml"
echo "2. Add 'crates/$COMPONENT' to workspace members"
echo "3. Try building: cargo build -p chromium-$COMPONENT"
```

### Template: Migration Validation Script

```bash
#!/bin/bash
# scripts/validate_migration.sh
# Validates a Cargo migration

set -e

COMPONENT=$1
if [ -z "$COMPONENT" ]; then
    echo "Usage: $0 <component_name>"
    exit 1
fi

echo "=== Validating $COMPONENT migration ==="

# 1. Build check
echo "Building with Cargo..."
if cargo build -p chromium-$COMPONENT; then
    echo "✅ Cargo build successful"
else
    echo "❌ Cargo build failed"
    exit 1
fi

# 2. Test check
echo "Running tests..."
if cargo test -p chromium-$COMPONENT; then
    echo "✅ Tests passed"
else
    echo "❌ Tests failed"
    exit 1
fi

# 3. Clippy check
echo "Running clippy..."
if cargo clippy -p chromium-$COMPONENT -- -D warnings; then
    echo "✅ Clippy passed"
else
    echo "❌ Clippy found issues"
    exit 1
fi

# 4. Format check
echo "Checking formatting..."
if cargo fmt -p chromium-$COMPONENT -- --check; then
    echo "✅ Formatting correct"
else
    echo "⚠️  Needs formatting (run: cargo fmt)"
fi

# 5. Documentation check
echo "Generating documentation..."
if cargo doc -p chromium-$COMPONENT --no-deps; then
    echo "✅ Documentation generated"
else
    echo "❌ Documentation failed"
    exit 1
fi

echo "=== Validation complete ✅ ==="
```

## Testing Patterns

### Pattern 1: Unit Tests

```rust
// crates/COMPONENT_NAME/src/lib.rs

pub fn process_data(input: &[u8]) -> Vec<u8> {
    // Implementation
    input.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_data() {
        let input = b"test";
        let output = process_data(input);
        assert_eq!(output, input);
    }
    
    #[test]
    fn test_empty_input() {
        let output = process_data(&[]);
        assert!(output.is_empty());
    }
}
```

### Pattern 2: Integration Tests

```rust
// crates/COMPONENT_NAME/tests/integration_test.rs

use chromium_component_name::*;

#[test]
fn test_full_workflow() {
    // Setup
    let config = Config::default();
    let processor = Processor::new(config);
    
    // Execute
    let input = b"test data";
    let output = processor.process(input);
    
    // Verify
    assert!(!output.is_empty());
}
```

### Pattern 3: Benchmarks

```rust
// crates/COMPONENT_NAME/benches/benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chromium_component_name::process_data;

fn benchmark_process(c: &mut Criterion) {
    let input = vec![0u8; 1024];
    
    c.bench_function("process_1kb", |b| {
        b.iter(|| process_data(black_box(&input)))
    });
}

criterion_group!(benches, benchmark_process);
criterion_main!(benches);
```

## Resources

- [Implementation Guide](implementation_guide.md)
- [Cargo Basics](../training/cargo_basics.md)
- [Troubleshooting](../training/troubleshooting.md)
- [GN to Cargo Tool](../../../tools/cargo_migration/README.md)

---

**Usage**: Copy and customize these templates for your component migration
