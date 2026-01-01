# Tier 2 Migration Quick Reference

**Quick guide for migrating complex C++ FFI components to Cargo workspace**

---

## Prerequisites

- ✅ Phase 2 Tier 1 complete (pure Rust components migrated)
- ✅ Familiarity with cxx bridge for FFI
- ✅ Component depends on Chromium C++ infrastructure (//base, etc.)
- ✅ Read [TIER_2_PLANNING.md](TIER_2_PLANNING.md) for full context

## When to Use Tier 2 Migration

Use Tier 2 approach if your component:
- ☑️ Depends on Chromium base library (//base)
- ☑️ Requires GN-generated header files (buildflags, config)
- ☑️ Has complex C++ dependency chain
- ☑️ Needs platform-specific build configuration

If your component **doesn't** have these dependencies, use Tier 1 approach instead.

## Tier 2 Migration Checklist

### 1. Assessment Phase

- [ ] **Review component structure**
  - Identify Rust source files
  - Identify C++ wrapper files
  - List C++ dependencies (especially //base usage)
  - Check for GN-generated headers

- [ ] **Check build requirements**
  - Review existing BUILD.gn file
  - Identify build flags and configuration
  - Note platform-specific code
  - List external dependencies

- [ ] **Estimate complexity**
  - Simple: Minimal //base usage, few C++ deps (1-2 weeks)
  - Moderate: Some //base usage, moderate C++ deps (2-4 weeks)
  - Complex: Heavy //base usage, deep C++ deps (4-8 weeks)

### 2. Preparation Phase

- [ ] **Set up Cargo.toml**
  ```toml
  [package]
  name = "component-name"
  version.workspace = true
  edition.workspace = true
  license.workspace = true
  repository.workspace = true
  
  [lib]
  crate-type = ["staticlib", "rlib"]
  path = "lib.rs"
  
  [dependencies]
  cxx = "1.0"
  
  [build-dependencies]
  cxx-build = "1.0"
  ```

- [ ] **Create build.rs for hybrid build**
  ```rust
  use std::path::PathBuf;
  use std::env;
  
  fn main() {
      // Find workspace root
      let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
      let root_dir = PathBuf::from(&manifest_dir)
          .ancestors()
          .nth(4)  // Adjust based on depth
          .expect("Failed to find workspace root");
      
      // Check for GN output
      let out_dir = root_dir.join("out/Default");
      if !out_dir.exists() {
          eprintln!("WARNING: GN build directory not found");
          eprintln!("This component requires GN to have been run first.");
          eprintln!("Run: gn gen out/Default && ninja -C out/Default [targets]");
      }
      
      // Build cxx bridge
      let mut build = cxx_build::bridge("cxx.rs");
      build
          .file("wrapper.cc")
          .flag_if_supported("-std=c++17")
          .include(&root_dir);
      
      // Add GN-generated header paths
      if out_dir.exists() {
          build.include(out_dir.join("gen"));
      }
      
      build.compile("component_cxx");
      
      // Rerun triggers
      println!("cargo:rerun-if-changed=wrapper.cc");
      println!("cargo:rerun-if-changed=wrapper.h");
      println!("cargo:rerun-if-changed=cxx.rs");
  }
  ```

- [ ] **Add to workspace Cargo.toml**
  ```toml
  members = [
      # ... existing members ...
      "path/to/component",  # Your new Tier 2 component
  ]
  ```

### 3. Build Testing Phase

- [ ] **Run GN build first**
  ```bash
  # Generate GN build files
  gn gen out/Default
  
  # Build C++ dependencies
  ninja -C out/Default device/bluetooth  # Adjust target path
  ```

- [ ] **Test Cargo build**
  ```bash
  # Try building with Cargo
  cargo check -p component-name
  
  # Full build
  cargo build -p component-name
  ```

- [ ] **Use hybrid build script**
  ```bash
  # From repository root
  ./tools/cargo_migration/hybrid_build.sh
  ```

- [ ] **Debug build issues**
  - Check include paths in build.rs
  - Verify GN-generated headers exist in out/Default/gen
  - Review compiler errors for missing headers
  - Adjust build.rs include paths as needed

### 4. Testing Phase

- [ ] **Run existing tests**
  ```bash
  # Cargo tests (if any)
  cargo test -p component-name
  
  # GN tests
  ninja -C out/Default component_tests
  out/Default/component_tests
  ```

- [ ] **Verify FFI integration**
  - Test C++ calling into Rust
  - Test Rust calling into C++
  - Check data marshaling across FFI
  - Verify error handling

- [ ] **Performance validation**
  ```bash
  # Run benchmarks if available
  cargo bench -p component-name
  ```

### 5. Documentation Phase

- [ ] **Update component README**
  - Document hybrid build requirement
  - Provide clear build instructions
  - Note GN prerequisites
  - List known limitations

- [ ] **Document lessons learned**
  - What worked well
  - What was challenging
  - Tips for next migration
  - Update TIER_2_PLANNING.md

- [ ] **Create migration report**
  - Time spent
  - Complexity encountered
  - Build performance metrics
  - Recommendations

### 6. Integration Phase

- [ ] **Update CI/CD**
  - Add hybrid build step to CI
  - Test on all platforms
  - Add to nightly builds
  - Monitor for regressions

- [ ] **Team communication**
  - Announce completion
  - Share documentation
  - Offer support for questions
  - Demo the component

## Common Patterns

### Pattern 1: Simple FFI Wrapper

**Use when**: Component has simple C++ wrapper, minimal dependencies

```rust
// cxx.rs
#[cxx::bridge(namespace=component_bridge)]
pub mod ffi {
    unsafe extern "C++" {
        include!("component/wrapper.h");
        
        type ComponentHandle;
        fn create_component() -> UniquePtr<ComponentHandle>;
        fn process_data(handle: &ComponentHandle, data: &[u8]) -> bool;
    }
    
    extern "Rust" {
        fn rust_callback(result: i32);
    }
}
```

### Pattern 2: Complex Data Structures

**Use when**: Need to share complex data across FFI

```rust
// Define shared types
#[cxx::bridge]
pub mod ffi {
    struct SharedData {
        id: u64,
        values: Vec<i32>,
        name: String,
    }
    
    unsafe extern "C++" {
        fn process_shared_data(data: &SharedData);
    }
}
```

### Pattern 3: Opaque C++ Types

**Use when**: C++ types are too complex for FFI

```rust
// Use opaque pointers
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        type OpaqueType;  // Don't expose internal structure
        
        fn create_opaque() -> UniquePtr<OpaqueType>;
        fn use_opaque(obj: &OpaqueType);
    }
}
```

## Troubleshooting

### Error: "No such file or directory" for Chromium headers

**Cause**: GN hasn't generated required headers  
**Solution**: Run `gn gen out/Default && ninja -C out/Default [targets]` first

### Error: Linker can't find symbols

**Cause**: Missing C++ library linkage  
**Solution**: Add to build.rs:
```rust
println!("cargo:rustc-link-search=native=/path/to/libs");
println!("cargo:rustc-link-lib=static=libraryname");
```

### Error: "Undefined reference" to Rust functions

**Cause**: Cargo didn't build Rust library  
**Solution**: Ensure `crate-type = ["staticlib", "rlib"]` in Cargo.toml

### Build works locally but fails in CI

**Cause**: CI might not have GN outputs  
**Solution**: Update CI to run GN first, or use hybrid build script

## Best Practices

✅ **DO**:
- Use hybrid build script for consistency
- Document GN prerequisites clearly
- Keep FFI boundary thin and simple
- Test on all target platforms
- Version lock dependencies
- Add comprehensive comments

❌ **DON'T**:
- Expect pure Cargo build for Tier 2 components
- Expose complex C++ types across FFI
- Skip platform testing
- Forget to update CI/CD
- Ignore build warnings
- Rush the migration

## Resources

- **Planning**: [TIER_2_PLANNING.md](TIER_2_PLANNING.md)
- **Overall Status**: [../MIGRATION_STATUS.md](../MIGRATION_STATUS.md)
- **Hybrid Build**: [../../tools/cargo_migration/hybrid_build.sh](../../tools/cargo_migration/hybrid_build.sh)
- **cxx Documentation**: https://cxx.rs/
- **Chromium FFI Guide**: [../training/ffi_best_practices.md](../training/ffi_best_practices.md)

## Getting Help

- **Slack**: #rust-migration
- **Email**: rust-migration@chromium.org
- **Office Hours**: Mon 2-3pm PT, Thu 10-11am PT
- **Documentation**: https://chromium.org/rust/migration

---

**Last Updated**: 2026-01-01  
**Maintained By**: Rust Migration Team
