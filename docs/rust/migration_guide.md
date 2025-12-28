# C++ to Rust Migration Guide

[TOC]

## Overview

This guide provides practical, step-by-step instructions for migrating C++ components to Rust within the Chromium codebase. It complements the high-level [Rust Adoption Plan](../rust_adoption_plan.md) with concrete technical guidance.

## When to Migrate to Rust

### Good Candidates

✅ **Strong Candidates for Rust Migration:**

1. **Parser Implementations**
   - Protocol parsers (HTTP, WebSocket, etc.)
   - File format parsers (JSON, XML, images, etc.)
   - URL parsing and validation
   - *Why*: Memory safety in parsing untrusted data

2. **Data Validation and Sanitization**
   - Input validation
   - Content security policies
   - XSS prevention
   - *Why*: Security-critical with clear boundaries

3. **Cryptographic Operations**
   - Hashing
   - Encryption/decryption
   - Key management
   - *Why*: Security-critical, good Rust library support

4. **New Isolated Components**
   - New features with minimal C++ dependencies
   - Utility libraries
   - Self-contained services
   - *Why*: No migration burden, clean slate

5. **Bug-Prone Components**
   - Components with history of memory safety issues
   - Complex memory management
   - Manual buffer handling
   - *Why*: Rust eliminates entire classes of bugs

### Poor Candidates

❌ **Not Recommended for Rust Migration:**

1. **Heavily Templated C++ Code**
   - Generic programming with complex template metaprogramming
   - *Why*: Rust generics work differently, migration would be complex

2. **Code with Deep C++ Integration**
   - Core rendering engine internals
   - Heavily object-oriented hierarchies with virtual dispatch
   - *Why*: FFI overhead, architectural mismatch

3. **Performance-Critical Hot Paths with Minimal Safety Issues**
   - Well-optimized C++ with proven safety
   - *Why*: Migration risk outweighs benefit

4. **GUI Code with Heavy Framework Dependencies**
   - Views, widgets with extensive Chrome UI dependencies
   - *Why*: FFI complexity, limited benefit

5. **Stable, Well-Tested Code Without Security Concerns**
   - Low-complexity utilities
   - Thoroughly validated logic
   - *Why*: Don't fix what isn't broken

## Migration Process

### Step 1: Assessment and Planning

#### 1.1 Document Current Implementation

Create a migration proposal document covering:

```markdown
## Component Overview
- Name: [Component name]
- Location: [File paths]
- Purpose: [What it does]
- Size: [Lines of code]

## Current Architecture
- Entry points: [Public APIs]
- Dependencies: [What it depends on]
- Consumers: [What depends on it]
- Threading model: [Threading considerations]

## Migration Justification
- Security benefits: [Expected improvements]
- Performance impact: [Expected changes]
- Maintenance benefits: [Reduced complexity, etc.]

## Risk Assessment
- Breaking changes: [Potential API changes]
- Performance risks: [Areas of concern]
- Schedule impact: [Estimated effort]

## Success Criteria
- Functional: [Must maintain these behaviors]
- Performance: [Performance requirements]
- Security: [Security improvements expected]
```

#### 1.2 Analyze Dependencies

```bash
# Find all includes from the component
cd /path/to/component
grep -r "^#include" . --include="*.h" --include="*.cc" | sort -u

# Find all consumers
cd chromium/src
git grep "YourComponentHeader.h" --name-only
```

Document:
- Direct C++ dependencies (what your component uses)
- Reverse dependencies (what uses your component)
- Third-party libraries
- Platform-specific code

#### 1.3 Design FFI Boundary

Sketch the FFI interface:

```rust
// Rust side (component.rs)
#[cxx::bridge(namespace = "chromium::your_component")]
mod ffi {
    extern "C++" {
        // C++ types/functions you'll call from Rust
        include!("base/callback.h");
        // ...
    }
    
    extern "Rust" {
        // Rust functions exposed to C++
        type ComponentImpl;
        
        fn create_component() -> Box<ComponentImpl>;
        fn process(self: &ComponentImpl, data: &[u8]) -> Result<Vec<u8>>;
        fn destroy(self: Box<ComponentImpl>);
    }
}
```

### Step 2: Setup Build Configuration

#### 2.1 Create BUILD.gn File

```python
# your_component/BUILD.gn

import("//build/rust/rust_static_library.gni")

rust_static_library("your_component_rs") {
  crate_root = "lib.rs"
  sources = [
    "lib.rs",
    "component.rs",
    "types.rs",
  ]
  
  # Dependencies on other Rust crates
  deps = [
    "//third_party/rust/serde/v1:lib",
  ]
  
  # Optional: if using cxx
  cxx_bindings = [
    "component.rs",
  ]
  
  # Optional: allow unsafe (requires justification)
  # allow_unsafe = true
}

# C++ wrapper around Rust implementation
source_set("your_component_wrapper") {
  sources = [
    "your_component_wrapper.cc",
    "your_component_wrapper.h",
  ]
  
  deps = [
    ":your_component_rs",
    "//base",
  ]
}

# Tests
test("your_component_unittests") {
  sources = [
    "your_component_unittest.cc",
  ]
  
  deps = [
    ":your_component_wrapper",
    "//testing/gtest",
  ]
}
```

#### 2.2 Setup Rust Library Structure

```
your_component/
├── BUILD.gn
├── lib.rs              # Rust library root
├── component.rs        # Main implementation with FFI
├── types.rs            # Data types
├── error.rs            # Error types
├── tests.rs            # Rust tests
├── your_component_wrapper.h    # C++ header
├── your_component_wrapper.cc   # C++ wrapper implementation
└── your_component_unittest.cc  # C++ tests
```

### Step 3: Implement Rust Core

#### 3.1 Basic Module Structure

```rust
// lib.rs
#![deny(unsafe_code)]  // Enforce safety unless explicitly needed

mod component;
mod types;
mod error;

#[cfg(test)]
mod tests;

pub use component::*;
pub use types::*;
pub use error::*;
```

#### 3.2 Implement Core Logic

```rust
// component.rs
use crate::{types::*, error::*};

pub struct ComponentImpl {
    // Internal state
    state: ComponentState,
}

impl ComponentImpl {
    pub fn new() -> Self {
        Self {
            state: ComponentState::default(),
        }
    }
    
    pub fn process(&self, data: &[u8]) -> Result<Vec<u8>, ComponentError> {
        // Validate input
        if data.is_empty() {
            return Err(ComponentError::InvalidInput);
        }
        
        // Process data
        let result = self.process_internal(data)?;
        
        Ok(result)
    }
    
    fn process_internal(&self, data: &[u8]) -> Result<Vec<u8>, ComponentError> {
        // Implementation details
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        let component = ComponentImpl::new();
        let result = component.process(b"test data");
        assert!(result.is_ok());
    }
}
```

#### 3.3 Define Error Types

```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Invalid input")]
    InvalidInput,
    
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// FFI-safe error codes
#[repr(i32)]
pub enum ErrorCode {
    Ok = 0,
    InvalidInput = 1,
    ProcessingFailed = 2,
    IoError = 3,
    UnknownError = 99,
}

impl From<&ComponentError> for ErrorCode {
    fn from(err: &ComponentError) -> Self {
        match err {
            ComponentError::InvalidInput => ErrorCode::InvalidInput,
            ComponentError::ProcessingFailed(_) => ErrorCode::ProcessingFailed,
            ComponentError::IoError(_) => ErrorCode::IoError,
        }
    }
}
```

### Step 4: Create FFI Layer

#### 4.1 Rust FFI Implementation

```rust
// component.rs (continued)
#[cxx::bridge(namespace = "chromium::your_component")]
mod ffi {
    #[repr(i32)]
    enum ErrorCode {
        Ok = 0,
        InvalidInput = 1,
        ProcessingFailed = 2,
        IoError = 3,
        UnknownError = 99,
    }
    
    extern "Rust" {
        type ComponentImpl;
        
        fn create_component() -> Box<ComponentImpl>;
        
        fn process_data(
            component: &ComponentImpl,
            input: &[u8],
            output: &mut Vec<u8>,
        ) -> ErrorCode;
    }
}

// FFI implementation
pub fn create_component() -> Box<ComponentImpl> {
    Box::new(ComponentImpl::new())
}

pub fn process_data(
    component: &ComponentImpl,
    input: &[u8],
    output: &mut Vec<u8>,
) -> ffi::ErrorCode {
    match component.process(input) {
        Ok(result) => {
            *output = result;
            ffi::ErrorCode::Ok
        }
        Err(e) => (&e).into(),
    }
}
```

#### 4.2 C++ Wrapper Implementation

```cpp
// your_component_wrapper.h
#ifndef YOUR_COMPONENT_WRAPPER_H_
#define YOUR_COMPONENT_WRAPPER_H_

#include <memory>
#include <vector>
#include "base/component_export.h"

// Forward declare the Rust type
namespace chromium::your_component {
struct ComponentImpl;
}

namespace chromium {

class COMPONENT_EXPORT YourComponent {
 public:
  YourComponent();
  ~YourComponent();
  
  // Not copyable or movable (Rust Box ownership)
  YourComponent(const YourComponent&) = delete;
  YourComponent& operator=(const YourComponent&) = delete;
  
  bool Process(const std::vector<uint8_t>& input,
               std::vector<uint8_t>* output);
  
 private:
  std::unique_ptr<your_component::ComponentImpl> impl_;
};

}  // namespace chromium

#endif  // YOUR_COMPONENT_WRAPPER_H_
```

```cpp
// your_component_wrapper.cc
#include "your_component/your_component_wrapper.h"
#include "your_component/component.rs.h"  // Generated by cxx

namespace chromium {

YourComponent::YourComponent() 
    : impl_(your_component::create_component()) {}

YourComponent::~YourComponent() = default;

bool YourComponent::Process(const std::vector<uint8_t>& input,
                            std::vector<uint8_t>* output) {
  DCHECK(output);
  
  rust::Slice<const uint8_t> input_slice(input.data(), input.size());
  rust::Vec<uint8_t> output_vec;
  
  auto error_code = your_component::process_data(
      *impl_, input_slice, output_vec);
  
  if (error_code != your_component::ErrorCode::Ok) {
    return false;
  }
  
  output->assign(output_vec.begin(), output_vec.end());
  return true;
}

}  // namespace chromium
```

### Step 5: Port Tests

#### 5.1 Rust Unit Tests

```rust
// tests.rs
use crate::*;

#[test]
fn test_empty_input() {
    let component = ComponentImpl::new();
    let result = component.process(&[]);
    assert!(result.is_err());
}

#[test]
fn test_valid_input() {
    let component = ComponentImpl::new();
    let input = b"valid test data";
    let result = component.process(input);
    assert!(result.is_ok());
}

#[test]
fn test_large_input() {
    let component = ComponentImpl::new();
    let input = vec![0u8; 1024 * 1024];  // 1MB
    let result = component.process(&input);
    assert!(result.is_ok());
}
```

#### 5.2 C++ Integration Tests

```cpp
// your_component_unittest.cc
#include "your_component/your_component_wrapper.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace chromium {

class YourComponentTest : public testing::Test {
 protected:
  void SetUp() override {
    component_ = std::make_unique<YourComponent>();
  }
  
  std::unique_ptr<YourComponent> component_;
};

TEST_F(YourComponentTest, ProcessValidInput) {
  std::vector<uint8_t> input = {'t', 'e', 's', 't'};
  std::vector<uint8_t> output;
  
  EXPECT_TRUE(component_->Process(input, &output));
  EXPECT_FALSE(output.empty());
}

TEST_F(YourComponentTest, ProcessEmptyInput) {
  std::vector<uint8_t> input;
  std::vector<uint8_t> output;
  
  EXPECT_FALSE(component_->Process(input, &output));
}

}  // namespace chromium
```

### Step 6: Performance Validation

#### 6.1 Benchmark Setup

```rust
// benches/benchmark.rs (if using Criterion)
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use your_component::ComponentImpl;

fn benchmark_process(c: &mut Criterion) {
    let component = ComponentImpl::new();
    let input = vec![0u8; 1024];
    
    c.bench_function("process 1KB", |b| {
        b.iter(|| {
            let _ = component.process(black_box(&input));
        })
    });
}

criterion_group!(benches, benchmark_process);
criterion_main!(benches);
```

#### 6.2 Comparative Benchmarking

```cpp
// your_component_benchmark.cc
#include "base/timer/elapsed_timer.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "testing/perf/perf_result_reporter.h"
#include "your_component/your_component_wrapper.h"

namespace chromium {

class YourComponentPerfTest : public testing::Test {
 protected:
  perf_test::PerfResultReporter SetUpReporter(const std::string& story) {
    perf_test::PerfResultReporter reporter("YourComponent", story);
    reporter.RegisterImportantMetric("time", "us");
    return reporter;
  }
};

TEST_F(YourComponentPerfTest, ProcessLatency) {
  auto reporter = SetUpReporter("ProcessLatency");
  
  YourComponent component;
  std::vector<uint8_t> input(1024, 0);
  std::vector<uint8_t> output;
  
  // Warmup
  for (int i = 0; i < 100; ++i) {
    component.Process(input, &output);
  }
  
  // Measure
  base::ElapsedTimer timer;
  for (int i = 0; i < 1000; ++i) {
    component.Process(input, &output);
  }
  
  reporter.AddResult("time", timer.Elapsed().InMicrosecondsF() / 1000);
}

}  // namespace chromium
```

### Step 7: Gradual Migration

#### 7.1 Feature Flag Integration

```cpp
// your_component_wrapper.cc
#include "base/feature_list.h"

BASE_FEATURE(kUseRustComponent,
             "UseRustComponent",
             base::FEATURE_DISABLED_BY_DEFAULT);

bool YourComponent::Process(const std::vector<uint8_t>& input,
                            std::vector<uint8_t>* output) {
  if (base::FeatureList::IsEnabled(kUseRustComponent)) {
    // Rust implementation
    return ProcessRust(input, output);
  } else {
    // Legacy C++ implementation
    return ProcessCpp(input, output);
  }
}
```

#### 7.2 A/B Testing

```cpp
// Use Chromium's field trial system
#include "base/metrics/field_trial.h"

bool ShouldUseRustImplementation() {
  const std::string group = 
      base::FieldTrialList::FindFullName("RustComponentTrial");
  return group == "Enabled";
}
```

### Step 8: Documentation and Handoff

#### 8.1 Code Documentation

```rust
//! Your Component
//!
//! This component provides [brief description].
//!
//! # Examples
//!
//! ```
//! use your_component::ComponentImpl;
//!
//! let component = ComponentImpl::new();
//! let result = component.process(b"data").unwrap();
//! ```
//!
//! # Safety
//!
//! This component uses only safe Rust. All FFI boundaries are carefully
//! validated.

/// Processes input data and returns result.
///
/// # Arguments
///
/// * `data` - Input data to process
///
/// # Returns
///
/// Processed data on success, error on failure.
///
/// # Errors
///
/// Returns `ComponentError::InvalidInput` if input is empty or invalid.
pub fn process(&self, data: &[u8]) -> Result<Vec<u8>, ComponentError> {
    // ...
}
```

#### 8.2 Migration Document

Create `MIGRATION.md` in component directory:

```markdown
# Your Component - Migration to Rust

## Overview
[Brief description of what was migrated and why]

## Architecture Changes
[Describe new architecture with diagrams if helpful]

## API Changes
[Document any API changes for consumers]

## Performance Impact
[Benchmark results comparing before/after]

## Known Issues
[Any known issues or limitations]

## Rollback Plan
[How to rollback if needed]

## Contact
[Team contact for questions]
```

## Common Patterns

### Pattern 1: Owned Data Transfer

```rust
// Rust owns the data, C++ gets a copy
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn process_owned(input: Vec<u8>) -> Vec<u8>;
    }
}

fn process_owned(input: Vec<u8>) -> Vec<u8> {
    // Process and return new Vec
    input.into_iter().map(|b| b.wrapping_add(1)).collect()
}
```

### Pattern 2: Borrowed Data

```rust
// Zero-copy processing with borrowed data
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn process_borrowed(input: &[u8], output: &mut Vec<u8>);
    }
}

fn process_borrowed(input: &[u8], output: &mut Vec<u8>) {
    output.clear();
    output.extend(input.iter().map(|b| b.wrapping_add(1)));
}
```

### Pattern 3: Callbacks

```rust
// Callback from Rust to C++
#[cxx::bridge]
mod ffi {
    extern "C++" {
        type ProgressCallback;
        fn on_progress(self: &ProgressCallback, percent: u32);
    }
    
    extern "Rust" {
        fn process_with_callback(
            input: &[u8],
            callback: &ProgressCallback,
        ) -> Result<()>;
    }
}

fn process_with_callback(
    input: &[u8],
    callback: &ffi::ProgressCallback,
) -> Result<()> {
    for (i, chunk) in input.chunks(100).enumerate() {
        // Process chunk
        let percent = ((i + 1) * 100 / input.len()) as u32;
        callback.on_progress(percent);
    }
    Ok(())
}
```

### Pattern 4: Opaque Types

```rust
// Hide implementation details from C++
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type OpaqueComponent;
        
        fn create_opaque() -> Box<OpaqueComponent>;
        fn call_method(self: &OpaqueComponent, arg: i32) -> i32;
    }
}

pub struct OpaqueComponent {
    // Internal details hidden from C++
    internal_state: ComplexRustType,
}
```

### Pattern 5: Error Propagation

```rust
// Convert Rust errors to C++-friendly status codes
#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum StatusCode {
        Ok = 0,
        InvalidInput = 1,
        NotFound = 2,
        InternalError = 99,
    }
    
    extern "Rust" {
        fn try_operation(input: &str) -> StatusCode;
    }
}

fn try_operation(input: &str) -> ffi::StatusCode {
    match try_operation_impl(input) {
        Ok(_) => ffi::StatusCode::Ok,
        Err(e) => match e {
            Error::InvalidInput => ffi::StatusCode::InvalidInput,
            Error::NotFound => ffi::StatusCode::NotFound,
            _ => ffi::StatusCode::InternalError,
        },
    }
}

fn try_operation_impl(input: &str) -> Result<(), Error> {
    // Implementation using ? operator
    validate_input(input)?;
    find_resource(input)?;
    Ok(())
}
```

## Troubleshooting

### Build Issues

**Problem**: `cxx` generated files not found
```
Solution: Ensure BUILD.gn has:
  cxx_bindings = [ "your_file.rs" ]
```

**Problem**: Linker errors with Rust symbols
```
Solution: Check that rust_static_library is in deps:
  deps = [ ":your_component_rs" ]
```

**Problem**: Build is very slow
```
Solution: 
- Use sccache
- Enable incremental compilation in .cargo/config.toml
- Reduce dependencies
```

### Runtime Issues

**Problem**: Segfault at FFI boundary
```
Debug checklist:
1. Verify lifetime management (no dangling references)
2. Check for data races with shared mutable state
3. Verify string encoding (UTF-8 vs. platform encoding)
4. Use AddressSanitizer to identify issue
```

**Problem**: Memory leak
```
Debug checklist:
1. Verify Box<T> ownership (created in Rust, destroyed in Rust)
2. Check for reference cycles
3. Ensure callbacks don't capture environment incorrectly
4. Use Valgrind or LeakSanitizer
```

**Problem**: Performance regression
```
Debug checklist:
1. Profile with perf or Instruments
2. Check for excessive copying at FFI boundary
3. Verify optimization level (should be release mode)
4. Look for allocation hot spots
```

## Best Practices Summary

### Do's ✅

- Start with small, isolated components
- Write comprehensive tests before migration
- Benchmark performance early and often
- Document FFI boundaries clearly
- Use `cxx` for type-safe FFI
- Minimize unsafe code
- Get security review for security-critical components
- Create detailed migration documentation
- Use feature flags for gradual rollout

### Don'ts ❌

- Don't migrate everything at once
- Don't ignore performance implications
- Don't skip testing
- Don't use unstable features without approval
- Don't create tight coupling between Rust and C++
- Don't expose complex Rust types directly to C++
- Don't skip code review
- Don't forget about platform-specific considerations

## Getting Help

- **Mailing list**: `rust-dev@chromium.org`
- **Slack**: `#rust` channel
- **Office hours**: [Check team calendar]
- **Rust working group**: [Contact information]
- **Documentation**: `//docs/rust/`

## Related Documentation

- [Rust in Chromium](../rust.md)
- [Rust FFI Guide](ffi.md)
- [Rust Adoption Plan](../rust_adoption_plan.md)
- [Third-party Crates](../../third_party/rust/README-importing-new-crates.md)
