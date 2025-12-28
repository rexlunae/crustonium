// Build script for C++/Rust integration prototype
// Phase 1.1: Research and Prototyping
//
// This demonstrates:
// 1. Building C++ code with the `cc` crate
// 2. Integrating cxx for FFI
// 3. Platform-specific build configuration

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=cpp/");
    println!("cargo:rerun-if-changed=src/ffi.rs");

    // Build C++ code using the cc crate
    build_cpp_code();

    // Build cxx bridge for FFI
    build_cxx_bridge();

    // Platform-specific linking
    configure_linking();
}

fn build_cpp_code() {
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .file("cpp/legacy_component.cc")
        .file("cpp/utilities.cc")
        .flag_if_supported("-std=c++17")
        .warnings(true);

    // Platform-specific flags
    if cfg!(target_os = "windows") {
        build.flag("/EHsc"); // Enable C++ exception handling
    } else if cfg!(target_os = "macos") {
        build.flag("-stdlib=libc++");
        build.flag("-Wall");
        build.flag("-Wextra");
    } else {
        // Linux and others
        build.flag("-Wall");
        build.flag("-Wextra");
    }

    // Compile the C++ code
    build.compile("legacy_cpp");
}

fn build_cxx_bridge() {
    let mut bridge = cxx_build::bridge("src/ffi.rs");
    
    bridge
        .file("cpp/bridge_impl.cc")
        .include("cpp")  // Add cpp directory to include path
        .flag_if_supported("-std=c++17");
    
    // Platform-specific flags for cxx bridge
    if cfg!(target_os = "macos") {
        bridge.flag("-stdlib=libc++");
    } else if cfg!(target_os = "windows") {
        bridge.flag("/EHsc"); // Enable C++ exception handling
    } else {
        // Linux and others
        bridge.flag("-Wall");
        bridge.flag("-Wextra");
    }
    
    bridge.compile("cpp_bridge");
}

fn configure_linking() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=stdc++");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=c++");
        }
        "windows" => {
            // MSVC automatically links C++ runtime
        }
        _ => {}
    }
}
