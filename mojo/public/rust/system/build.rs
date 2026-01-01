// Build script for mojo-rust-system-api (Tier 2 component)
//
// This component requires GN-generated Rust bindings for the Mojo C API.
// The bindings are generated via the chromium::import! macro which relies
// on the GN build system.
//
// For now, this build.rs just checks that we're in a Chromium build environment.
// Future: May need to verify GN output exists.

use std::env;
use std::path::PathBuf;

fn main() {
    // Get repository root (4 levels up from mojo/public/rust/system)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find repository root");

    // Check if we're in a Chromium build environment by looking for out/Default
    let gn_out = repo_root.join("out").join("Default");
    
    if !gn_out.exists() {
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("ERROR: GN build output not found");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!();
        eprintln!("This component requires a hybrid build approach:");
        eprintln!();
        eprintln!("  1. Generate GN build files:");
        eprintln!("     gn gen out/Default");
        eprintln!();
        eprintln!("  2. Build Mojo C API and bindings:");
        eprintln!("     ninja -C out/Default mojo/public/rust:mojo_c_system_bindings");
        eprintln!();
        eprintln!("  3. Then build with Cargo:");
        eprintln!("     cargo build -p mojo-rust-system-api");
        eprintln!();
        eprintln!("Or use the helper script:");
        eprintln!("  ./mojo/public/rust/system/build_hybrid.sh");
        eprintln!();
        eprintln!("See mojo/public/rust/system/README.md for details.");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Don't fail the build in case GN files are elsewhere
        // Just warn the developer
        println!("cargo:warning=GN build required for this component");
    } else {
        println!("cargo:rerun-if-changed={}", gn_out.display());
    }

    // Tell cargo to rerun if build files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib.rs");
    println!("cargo:rerun-if-changed=ffi.rs");
}
