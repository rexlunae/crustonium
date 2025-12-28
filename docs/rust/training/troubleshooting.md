# Cargo Troubleshooting Guide

**Phase 1.3: Documentation and Training**

Solutions to common problems when using Cargo in the Chromium build system.

[TOC]

## Build Errors

### "Could not compile X"

**Symptoms**:
```
error: could not compile `chromium-component`
```

**Common Causes**:

**1. Syntax Error**
```
error[E0425]: cannot find value `x` in this scope
```
**Solution**: Fix the Rust code error indicated

**2. Missing Dependency**
```
error[E0463]: can't find crate for `serde`
```
**Solution**: Add to Cargo.toml:
```toml
[dependencies]
serde = { workspace = true }
```

**3. Version Conflict**
```
error: failed to select a version for `serde`
```
**Solution**: Use workspace dependencies:
```toml
# In root Cargo.toml
[workspace.dependencies]
serde = "1.0"

# In member
[dependencies]
serde = { workspace = true }
```

### "Linking with `cc` failed"

**Symptoms**:
```
error: linking with `cc` failed: exit status: 1
= note: /usr/bin/ld: cannot find -lstdc++
```

**Solutions**:

**1. Missing C++ library**
```bash
# Ubuntu/Debian
sudo apt install libstdc++-12-dev

# macOS (usually included)
xcode-select --install
```

**2. Wrong linker**
Add to `.cargo/config.toml`:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

**3. Missing library in build.rs**
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=stdc++");
}
```

### "cxx bridge failed"

**Symptoms**:
```
error: failed to run custom build command for `my-crate`
fatal error: 'my_header.h' file not found
```

**Solutions**:

**1. Missing include path**
```rust
// build.rs
fn main() {
    cxx_build::bridge("src/ffi.rs")
        .file("cpp/impl.cc")
        .include("cpp")  // Add this!
        .compile("bridge");
}
```

**2. Wrong include path in FFI**
```rust
// ffi.rs
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("my_header.h");  // Relative to include path
    }
}
```

**3. C++ compilation error**
Check the actual C++ error in the output and fix the C++ code.

## Dependency Issues

### "Failed to download dependencies"

**Symptoms**:
```
error: failed to download from `https://...`
```

**Solutions**:

**1. Network issue**
```bash
# Retry
cargo clean
cargo build

# Use alternative registry
export CARGO_HTTP_MULTIPLEXING=false
```

**2. Corporate proxy**
```bash
# Set proxy
export HTTP_PROXY=http://proxy:8080
export HTTPS_PROXY=http://proxy:8080

# Or in .cargo/config.toml
[http]
proxy = "http://proxy:8080"
```

**3. Use vendored dependencies**
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "third_party/rust"
```

### "Version resolution failed"

**Symptoms**:
```
error: failed to select a version for `package`
  required by package `A`
  required by package `B`
```

**Solution**: Use exact compatible versions in workspace:
```toml
[workspace.dependencies]
problematic-crate = "=1.0.5"  # Pin exact version
```

### "Cargo.lock is out of date"

**Symptoms**:
```
error: the lock file needs to be updated
```

**Solution**:
```bash
# Update lock file
cargo update

# Or for specific package
cargo update -p package-name
```

## Workspace Issues

### "Package not found in workspace"

**Symptoms**:
```
error: package `chromium-component` not found in workspace
```

**Solutions**:

**1. Not in members list**
```toml
# Add to root Cargo.toml
[workspace]
members = [
    "crates/component",  # Add this
]
```

**2. Wrong path**
Verify the path exists and Cargo.toml is there:
```bash
ls crates/component/Cargo.toml
```

**3. Excluded**
```toml
[workspace]
exclude = [
    "crates/component",  # Remove if you want it included
]
```

### "Circular dependency detected"

**Symptoms**:
```
error: cyclic package dependency: package `A` depends on itself
```

**Solution**: Refactor to break cycle
```
Before:
  A -> B -> A  (circular!)

After:
  A -> C
  B -> C
```

Extract common code to new crate C.

## Build Performance

### "Builds are too slow"

**Solutions**:

**1. Use sccache**
```bash
# Install
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache

# Verify
sccache --show-stats
```

**2. Use cargo check instead of build**
```bash
# Much faster, no linking
cargo check
```

**3. Reduce dependencies**
```bash
# Find what's slow
cargo build --timings

# Remove unused deps
cargo install cargo-udeps
cargo udeps
```

**4. Use release mode for dependencies**
```toml
# In Cargo.toml
[profile.dev.package."*"]
opt-level = 2  # Optimize deps in dev builds
```

### "Incremental builds aren't working"

**Symptoms**: Every build seems like a clean build

**Solutions**:

**1. Check incremental is enabled**
```toml
# .cargo/config.toml
[build]
incremental = true
```

**2. Clean corrupted incremental state**
```bash
rm -rf target/debug/incremental
cargo build
```

**3. Check disk space**
```bash
df -h .
# Incremental needs temp space
```

## Test Issues

### "Tests are failing in CI but pass locally"

**Causes & Solutions**:

**1. Different Rust version**
```bash
# Pin Rust version in rust-toolchain.toml
[toolchain]
channel = "1.75.0"
```

**2. Platform differences**
```rust
#[test]
#[cfg(target_os = "linux")]
fn linux_specific_test() {
    // Only runs on Linux
}
```

**3. Missing test fixtures**
Ensure test data is committed:
```bash
git add tests/fixtures/
```

### "Test fails with 'too many open files'"

**Solution**:
```bash
# Increase limit (Linux/macOS)
ulimit -n 4096

# Or run tests serially
cargo test -- --test-threads=1
```

### "Doc tests fail but unit tests pass"

**Cause**: Example code in doc comments is wrong

**Solution**: Fix or mark as no_run:
```rust
/// # Examples
///
/// ```no_run
/// // This won't be executed
/// dangerous_operation();
/// ```
```

## FFI and C++ Integration

### "Undefined symbol errors"

**Symptoms**:
```
undefined reference to `SomeClass::method()`
```

**Solutions**:

**1. Missing library**
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=mylib");
}
```

**2. Wrong name mangling**
```cpp
// C++ header
extern "C" {
    void my_function();  // C linkage
}
```

**3. Missing object file**
```rust
// build.rs
cc::Build::new()
    .file("cpp/missing.cc")  // Add this
    .compile("mylib");
```

### "cxx bridge type mismatch"

**Symptoms**:
```
error: type mismatch in bridge
```

**Solution**: Ensure C++ and Rust types match:
```rust
// Rust
fn process(data: &[u8]) -> String;

// C++
std::string process(rust::Slice<const uint8_t> data);
```

## Platform-Specific Issues

### Windows: "Long path names"

**Symptoms**:
```
error: Couldn't read ... filename too long
```

**Solution**:
```toml
# .cargo/config.toml
[build]
target-dir = "C:/t"  # Shorter path
```

Or enable long paths in Windows.

### macOS: "xcrun: error"

**Symptoms**:
```
xcrun: error: unable to find utility "cc"
```

**Solution**:
```bash
xcode-select --install
```

### Linux: "cannot find -lgcc_s"

**Solution**:
```bash
# Install build essentials
sudo apt install build-essential
```

## Hybrid Build Issues

### "GN and Cargo produce different outputs"

**Diagnostic**:
```bash
# Build with GN
./tools/cargo_migration/hybrid_build.sh --system gn
# Check output

# Build with Cargo
./tools/cargo_migration/hybrid_build.sh --system cargo
# Compare outputs
```

**Common Causes**:
1. Different optimization levels
2. Different feature flags
3. Different dependencies

**Solution**: Align build configurations

### "Hybrid build fails"

**Symptoms**: Either GN or Cargo part fails

**Solution**:
```bash
# Test each separately
./tools/cargo_migration/hybrid_build.sh --system gn
./tools/cargo_migration/hybrid_build.sh --system cargo

# Fix the failing one first
```

## Clean Build Tricks

### "Nothing else works"

**Nuclear option**:
```bash
# Clean everything
cargo clean
rm -rf target/
rm Cargo.lock

# Rebuild from scratch
cargo build
```

### "Corrupted registry cache"

```bash
# Clear cargo cache
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git

# Refetch
cargo build
```

## Diagnostic Commands

### Show build details

```bash
# Verbose build
cargo build --verbose

# Show exact commands
cargo build -vv

# Build timing report
cargo build --timings

# Show dependency tree
cargo tree

# Show duplicates
cargo tree --duplicates
```

### Check configuration

```bash
# Show current config
cargo config get

# Show build config
cargo config get build

# Show target config
cargo config get target.x86_64-unknown-linux-gnu
```

### Verify environment

```bash
# Check Rust version
rustc --version
cargo --version

# Check available targets
rustup show

# Check toolchain
rustup toolchain list
```

## Getting Help

### Before Asking for Help

1. **Check the error message carefully**
   - Often tells you exactly what's wrong
   
2. **Try with verbose output**
   ```bash
   cargo build --verbose
   ```

3. **Check recent changes**
   ```bash
   git diff HEAD~1 Cargo.toml
   ```

4. **Search existing issues**
   - Internal issue tracker
   - rust-lang/cargo GitHub issues

### When Asking for Help

**Include**:
- ✅ Exact error message (use code blocks)
- ✅ Cargo.toml contents
- ✅ Platform (OS, Rust version)
- ✅ What you've already tried
- ✅ Minimal reproduction if possible

**Example Good Question**:
```
I'm getting this error when building chromium-component:

error: linking with `cc` failed: exit status: 1
  = note: cannot find -lstdc++

Platform: Ubuntu 22.04
Rust: 1.75.0
Cargo.toml: [paste contents]

I've tried:
- sudo apt install libstdc++-12-dev
- Adding println!("cargo:rustc-link-lib=stdc++") to build.rs

Neither worked. Any ideas?
```

### Channels

- **Slack**: `#cargo-migration`
- **Email**: `build-dev@chromium.org`
- **Documentation**: `/docs/cargo_adoption_plan.md`
- **Office Hours**: Weekly (check team calendar)

## Quick Reference

### Most Common Fixes

```bash
# 1. Dependency issue
cargo update

# 2. Corrupted build
cargo clean && cargo build

# 3. Incremental build issue
rm -rf target/debug/incremental

# 4. Lock file out of date
cargo update

# 5. Missing dependency
# Add to Cargo.toml [dependencies]

# 6. Wrong Rust version
rustup default stable
rustup update
```

## Resources

- [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html)
- [Cargo Troubleshooting](https://doc.rust-lang.org/cargo/guide/troubleshooting.html)
- [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [Chromium Cargo Basics](cargo_basics.md)
- [Workspace Best Practices](workspace_best_practices.md)

---

**Can't find your issue?** Ask in `#cargo-migration` on Slack!
