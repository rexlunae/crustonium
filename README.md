# ![Logo](chrome/app/theme/chromium/product_logo_64.png) Chromium

Chromium is an open-source browser project that aims to build a safer, faster,
and more stable way for all users to experience the web.

The project's web site is https://www.chromium.org.

To check out the source code locally, don't use `git clone`! Instead,
follow [the instructions on how to get the code](docs/get_the_code.md).

Documentation in the source is rooted in [docs/README.md](docs/README.md).

Learn how to [Get Around the Chromium Source Code Directory
Structure](https://www.chromium.org/developers/how-tos/getting-around-the-chrome-source-code).

For historical reasons, there are some small top level directories. Now the
guidance is that new top level directories are for product (e.g. Chrome,
Android WebView, Ash). Even if these products have multiple executables, the
code should be in subdirectories of the product.

If you found a bug, please file it at https://crbug.com/new.

## Rust Development

This repository includes Rust components that can be built using Cargo alongside
the traditional GN/Ninja build system. For Rust development:

- **Quick Start**: See [docs/RUST_COMPILATION_QUICK_REF.md](docs/RUST_COMPILATION_QUICK_REF.md)
- **Detailed Guide**: See [docs/RUST_COMPILATION.md](docs/RUST_COMPILATION.md)
- **Build Rust components**: `cargo build --workspace`
- **Run tests**: `cargo test --workspace`

### Rust Migration Status

**Phase 2 Tier 1 Complete** ✅ (2026-01-01)

We have successfully migrated **6 production components** to the Cargo workspace:
- Testing infrastructure (`rust_gtest_interop`)
- Build tools (`gnrt`)
- QR code generator
- Payment validation
- Data import utilities
- Media filters

**Current Status**: Ready for Phase 2 Tier 2 (complex C++ FFI integration)

For migration progress and strategy:
- **Migration Status**: [docs/rust/MIGRATION_STATUS.md](docs/rust/MIGRATION_STATUS.md)
- **Overall Strategy**: [docs/rust_adoption_plan.md](docs/rust_adoption_plan.md)
- **Cargo Adoption**: [docs/cargo_adoption_plan.md](docs/cargo_adoption_plan.md)
