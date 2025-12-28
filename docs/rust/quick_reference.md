# Rust Quick Reference for Chromium Developers

[TOC]

## Overview

This document provides quick answers to common questions when working with Rust in Chromium. For comprehensive guidance, see:
- [Rust in Chromium](../rust.md) - General Rust usage
- [Rust FFI Guide](ffi.md) - C++/Rust interop
- [Migration Guide](migration_guide.md) - Migrating C++ to Rust
- [Rust Adoption Plan](../rust_adoption_plan.md) - Strategic roadmap

## Quick Start

### Should I Use Rust for This?

**Use Rust if:**
- ✅ Handling untrusted data (parsers, validators)
- ✅ Creating a new isolated component
- ✅ Component has memory safety concerns
- ✅ You have Rust expertise or learning time

**Use C++ if:**
- ❌ Deep integration with existing C++ frameworks
- ❌ Heavy use of templates or OOP patterns
- ❌ Performance-critical code without safety issues
- ❌ Team lacks Rust experience and time is tight

### Basic Setup

```bash
# 1. Create your Rust file
touch //your/component/lib.rs

# 2. Create BUILD.gn
cat > //your/component/BUILD.gn << 'EOF'
import("//build/rust/rust_static_library.gni")

rust_static_library("component_rs") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}
EOF

# 3. Build
autoninja -C out/Default your/component:component_rs
```

## Common Tasks

### Add a Rust Library

```python
# BUILD.gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_component") {
  crate_root = "src/lib.rs"
  sources = [
    "src/lib.rs",
    "src/module.rs",
  ]
  
  # Dependencies
  deps = [
    "//third_party/rust/serde/v1:lib",
  ]
  
  # For cxx FFI
  cxx_bindings = [ "src/ffi.rs" ]
  
  # Only if needed and justified
  # allow_unsafe = true
}
```

### Call Rust from C++

**Rust side:**
```rust
// ffi.rs
#[cxx::bridge(namespace = "my_component")]
mod ffi {
    extern "Rust" {
        fn greet(name: &str) -> String;
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

**C++ side:**
```cpp
// component.cc
#include "your/component/src/ffi.rs.h"

std::string result = my_component::greet("World");
// result = "Hello, World!"
```

### Call C++ from Rust

**C++ side:**
```cpp
// callback.h
namespace my_component {
void LogMessage(const std::string& message);
}  // namespace my_component
```

**Rust side:**
```rust
#[cxx::bridge(namespace = "my_component")]
mod ffi {
    unsafe extern "C++" {
        include!("your/component/callback.h");
        fn LogMessage(message: &str);
    }
}

fn my_function() {
    ffi::LogMessage("Called from Rust");
}
```

### Add Third-party Crate

```bash
# 1. Edit Cargo.toml
cd third_party/rust/chromium_crates_io
# Add to [dependencies]:
#   your_crate = "1.0"

# 2. Run gnrt
cd ../../..
./tools/crates/run_gnrt.py -- gen

# 3. Use in BUILD.gn
deps = [
  "//third_party/rust/your_crate/v1:lib",
]
```

See [importing crates guide](../../third_party/rust/README-importing-new-crates.md) for details.

### Error Handling

```rust
use thiserror::Error;

// Define errors
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Not found")]
    NotFound,
}

// Use Result
fn fallible_operation() -> Result<String, MyError> {
    // Use ? for error propagation
    let data = load_data()?;
    validate(data)?;
    Ok(data)
}

// FFI error conversion
#[repr(i32)]
pub enum StatusCode {
    Ok = 0,
    InvalidInput = 1,
    NotFound = 2,
}

impl From<MyError> for StatusCode {
    fn from(err: MyError) -> Self {
        match err {
            MyError::InvalidInput(_) => StatusCode::InvalidInput,
            MyError::NotFound => StatusCode::NotFound,
        }
    }
}
```

### Logging

```rust
use log::{debug, info, warn, error};

fn my_function() {
    debug!("Debug message");
    info!("Info message");
    warn!("Warning: {}", reason);
    error!("Error: {:?}", err);
    
    // Alternative: dbg! macro for quick debugging
    let value = dbg!(compute_value());
}
```

### Testing

**Rust unit tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic() {
        assert_eq!(add(2, 2), 4);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Expected panic");
    }
}
```

**Integration with gtest:**
```rust
use rust_gtest_interop::prelude::*;

#[gtest(MyTest, BasicTest)]
fn test_basic() {
    expect_eq!(2 + 2, 4);
}
```

**BUILD.gn for tests:**
```python
rust_unit_test("component_rust_unittests") {
  crate_root = "src/lib.rs"
  sources = [ "src/lib.rs" ]
}
```

## FFI Patterns

### Passing Data to Rust

```rust
// Slice (borrowed, zero-copy)
fn process_slice(data: &[u8]) -> usize {
    data.len()
}

// Vec (owned, moves data)
fn process_vec(data: Vec<u8>) -> Vec<u8> {
    data
}

// String slice
fn process_str(text: &str) -> usize {
    text.len()
}

// Mutable output parameter
fn fill_output(output: &mut Vec<u8>) {
    output.extend_from_slice(b"data");
}
```

### Returning Data from Rust

```rust
// Return owned Vec
fn create_vec() -> Vec<u8> {
    vec![1, 2, 3]
}

// Return String
fn create_string() -> String {
    "Hello".to_string()
}

// Return Result
fn try_parse(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e| format!("{}", e))
}

// Return boxed type (opaque to C++)
fn create_instance() -> Box<MyStruct> {
    Box::new(MyStruct::new())
}
```

### Opaque Types

```rust
// Rust struct hidden from C++
pub struct OpaqueType {
    internal: ComplexInternalState,
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type OpaqueType;
        
        fn create() -> Box<OpaqueType>;
        fn method(self: &OpaqueType, arg: i32) -> i32;
    }
}

fn create() -> Box<OpaqueType> {
    Box::new(OpaqueType::new())
}

fn method(self: &OpaqueType, arg: i32) -> i32 {
    self.internal.compute(arg)
}
```

### Shared Types

```rust
#[cxx::bridge]
mod ffi {
    // Shared struct (visible to both Rust and C++)
    struct SharedData {
        id: i32,
        name: String,
    }
    
    extern "Rust" {
        fn process_shared(data: SharedData) -> SharedData;
    }
}

fn process_shared(mut data: ffi::SharedData) -> ffi::SharedData {
    data.id += 1;
    data
}
```

## Build System Cheat Sheet

### Templates

| Template | Purpose | Example |
|----------|---------|---------|
| `rust_static_library` | Rust library for use in Chromium | Core implementation |
| `cargo_crate` | Third-party crate from crates.io | Dependencies |
| `rust_executable` | Rust binary | Tools, tests |
| `rust_unit_test` | Rust unit tests | Testing |
| `rust_bindgen` | Generate Rust bindings from C | Legacy C APIs |

### Common BUILD.gn Options

```python
rust_static_library("example") {
  # Required
  crate_root = "src/lib.rs"
  
  # Sources (automatically found if not specified)
  sources = [ "src/lib.rs", "src/other.rs" ]
  
  # Dependencies
  deps = [
    "//other/component:lib",
    "//third_party/rust/serde/v1:lib",
  ]
  
  # Features (from Cargo.toml)
  features = [ "feature_name" ]
  
  # Unsafe code (needs justification)
  allow_unsafe = true
  
  # Unstable features (needs approval)
  allow_unstable_features = true
  
  # C++ interop
  cxx_bindings = [ "src/ffi.rs" ]
  
  # Build script
  build_root = "build.rs"
  build_sources = [ "build.rs" ]
  
  # Conditional compilation
  if (is_win) {
    sources += [ "src/windows.rs" ]
  }
}
```

## Common Patterns

### Option and Result

```rust
// Option: might not have a value
let maybe: Option<i32> = Some(5);
if let Some(value) = maybe {
    println!("Got {}", value);
}

// Result: might fail
let result: Result<i32, String> = Ok(5);
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}

// Unwrap with default
let value = maybe.unwrap_or(0);
let value = result.unwrap_or_else(|_| 0);

// Question mark operator (propagate error)
fn process() -> Result<i32, MyError> {
    let x = operation1()?;  // Returns early if error
    let y = operation2()?;
    Ok(x + y)
}
```

### Iterators

```rust
// Transform data
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();

// Filter
let evens: Vec<i32> = (0..10)
    .filter(|x| x % 2 == 0)
    .collect();

// Find
let first_even = (0..10).find(|x| x % 2 == 0);

// Fold (reduce)
let sum = vec![1, 2, 3].iter().fold(0, |acc, x| acc + x);

// Chain operations
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect();
```

### Ownership and Borrowing

```rust
// Owned data
let owned = String::from("hello");
take_ownership(owned);  // owned is moved
// owned is no longer accessible

// Borrowed data (immutable)
let s = String::from("hello");
read_only(&s);  // s is borrowed
println!("{}", s);  // s is still accessible

// Mutable borrow
let mut s = String::from("hello");
modify(&mut s);  // s is mutably borrowed
println!("{}", s);  // s is still accessible

// Clone when needed
let s1 = String::from("hello");
let s2 = s1.clone();  // explicit copy
println!("{} {}", s1, s2);  // both accessible
```

### Common Collections

```rust
use std::collections::{HashMap, HashSet, VecDeque};

// Vec (dynamic array)
let mut vec = Vec::new();
vec.push(1);
vec.extend([2, 3]);

// HashMap (dictionary)
let mut map = HashMap::new();
map.insert("key", "value");
if let Some(value) = map.get("key") {
    println!("{}", value);
}

// HashSet (unique values)
let mut set = HashSet::new();
set.insert(1);
set.insert(1);  // duplicate ignored
assert_eq!(set.len(), 1);
```

## Debugging

### Print Debugging

```rust
// Debug formatting
println!("{:?}", value);  // Debug trait
println!("{:#?}", value);  // Pretty-print

// dbg! macro (prints and returns value)
let x = dbg!(compute_value());

// Custom debug
#[derive(Debug)]
struct MyStruct {
    field: i32,
}
```

### Using Debugger

```bash
# GDB
gdb --args out/Default/unit_tests --gtest_filter=YourTest.*

# LLDB
lldb -- out/Default/unit_tests --gtest_filter=YourTest.*

# Set breakpoint in Rust
(lldb) br set -n my_component::my_function
```

### Common Issues

**Issue**: `cannot borrow as mutable`
```rust
// Problem: multiple borrows
let x = &mut data;
let y = &mut data;  // ERROR

// Solution: use only one mutable borrow at a time
let x = &mut data;
use_borrow(x);
// x goes out of scope
let y = &mut data;  // OK
```

**Issue**: `moved value used after move`
```rust
// Problem: using value after move
let s = String::from("hello");
take_ownership(s);
println!("{}", s);  // ERROR: s was moved

// Solution: clone or borrow
let s = String::from("hello");
take_ownership(s.clone());
println!("{}", s);  // OK: s is still valid
```

**Issue**: Lifetime errors
```rust
// Usually means data doesn't live long enough
// Solution: ensure data outlives references
// or use owned data (String vs &str, Vec vs &[T])
```

## Performance Tips

### Avoid Allocations

```rust
// Reuse buffers
let mut buffer = Vec::with_capacity(1024);
loop {
    buffer.clear();
    fill_buffer(&mut buffer);
    process(&buffer);
}

// Use string builders
let mut s = String::with_capacity(100);
for i in 0..10 {
    s.push_str(&i.to_string());
}

// Use slices instead of Vecs when possible
fn process(data: &[u8]) {  // Borrowed
    // ...
}
```

### Iteration

```rust
// Prefer iteration over indexing
// Bad
for i in 0..vec.len() {
    process(vec[i]);  // Bounds check each time
}

// Good
for item in &vec {
    process(item);  // No bounds checks
}

// Even better with iterators
vec.iter().for_each(|item| process(item));
```

### Zero-cost Abstractions

```rust
// These have no runtime cost:
// - Iterators (compile to loops)
// - Closures (inline)
// - Pattern matching (compile to jumps)
// - Trait dispatch (static dispatch)

// Example: as fast as hand-written loop
let sum: i32 = (0..100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();
```

## Rust vs C++ Equivalents

| Rust | C++ | Notes |
|------|-----|-------|
| `Vec<T>` | `std::vector<T>` | Dynamic array |
| `String` | `std::string` | Owned string |
| `&str` | `std::string_view` | String slice |
| `&[T]` | `std::span<T>` | Array slice |
| `Box<T>` | `std::unique_ptr<T>` | Unique ownership |
| `Rc<T>` | `std::shared_ptr<T>` | Shared ownership (single-threaded) |
| `Arc<T>` | `std::shared_ptr<T>` | Shared ownership (thread-safe) |
| `Option<T>` | `std::optional<T>` | Maybe value |
| `Result<T, E>` | `absl::StatusOr<T>` | Result or error |
| `RefCell<T>` | Runtime borrow checking | Interior mutability |
| `trait` | abstract class / concepts | Interface |
| `impl Trait` | template / concepts | Generics |
| `match` | `switch` / visitor | Pattern matching |
| `if let` | - | Conditional destructuring |

## Getting Help

### Resources

- **Documentation**: `//docs/rust/`
- **Style guide**: `//styleguide/rust/rust.md`
- **FFI guide**: `//docs/rust/ffi.md`
- **Examples**: Look for `*.rs` files in `//components/`, `//media/`, etc.

### Community

- **Mailing list**: rust-dev@chromium.org
- **Slack**: #rust channel
- **Office hours**: Check team calendar
- **Code review**: Add rust-dev@chromium.org for Rust expertise

### External Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Comprehensive Rust (Chromium)](https://google.github.io/comprehensive-rust/chromium/)
- [cxx.rs](https://cxx.rs/)

## Checklist for Code Review

Before submitting Rust code for review:

- [ ] Code compiles without warnings
- [ ] `git cl format` applied
- [ ] Tests added and passing
- [ ] Error handling is appropriate
- [ ] FFI boundaries are documented
- [ ] No unsafe code (or justified if necessary)
- [ ] Performance validated if critical path
- [ ] Documentation comments for public APIs
- [ ] BUILD.gn dependencies are minimal
- [ ] Third-party crates have been reviewed

## Common Commands

```bash
# Format code
git cl format

# Build
autoninja -C out/Default your/component:target

# Run tests
out/Default/unit_tests --gtest_filter=YourTest.*

# Run Rust-specific tests
out/Default/your_component_rust_unittests

# Check for issues
./tools/crates/run_gnrt.py -- gen  # Update Rust deps
```

## FAQ

**Q: When should I use `unsafe`?**
A: Rarely. Only when interfacing with C APIs or for specific performance optimizations that can't be achieved otherwise. Always document why it's safe.

**Q: How do I handle strings across FFI?**
A: Use `&str` (Rust) / `rust::Str` (C++) for borrowing, `String` / `rust::String` for ownership. Be careful with UTF-8 vs platform encoding.

**Q: What about threading?**
A: Rust's `Send` and `Sync` traits ensure thread safety. Integrate with Chromium's task posting. Use `Arc` for shared ownership across threads.

**Q: How do I profile Rust code?**
A: Use standard profilers (perf, Instruments). Rust code appears in profiles with mangled names. Use `rustfilt` to demangle.

**Q: Can I use async/await?**
A: Chromium doesn't use Tokio or other async runtimes yet. Stick with synchronous code or Chromium's task posting for now.

---

**Remember**: When in doubt, ask on rust-dev@chromium.org or #rust Slack!
