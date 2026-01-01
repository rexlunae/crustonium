// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::path::PathBuf;
use std::env;

fn main() {
    // This component requires GN-generated headers from Chromium's build system.
    // For now, we require that GN has been run before building with Cargo.
    // This is a known limitation for Tier 2 components with deep C++ dependencies.
    
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&manifest_dir);
    
    // Find workspace root by looking for .gn marker file
    let root_dir = manifest_path
        .ancestors()
        .find(|p| p.join(".gn").exists())
        .expect("Failed to find workspace root (no .gn file found)");
    
    // Check for GN output directory
    let out_dir = root_dir.join("out/Default");
    if !out_dir.exists() {
        eprintln!("WARNING: GN build directory not found at {:?}", out_dir);
        eprintln!("This component requires GN to have been run first.");
        eprintln!("Please run: gn gen out/Default && ninja -C out/Default device/bluetooth");
        eprintln!("");
        eprintln!("For pure Cargo build support, use Phase 2 Tier 1 components.");
        // We'll still try to build, but it will likely fail
    }
    
    // Build the cxx bridge
    let mut build = cxx_build::bridge("cxx.rs");
    build
        .file("wrapper_functions.cc")
        .flag_if_supported("-std=c++17")
        .include(&root_dir);  // Root chromium directory
    
    // Add GN-generated header paths if they exist
    if out_dir.exists() {
        build.include(out_dir.join("gen"));
    }
    
    build.compile("ble_scan_parser_cxx");

    // Tell cargo to rerun if C++ files change
    println!("cargo:rerun-if-changed=wrapper_functions.cc");
    println!("cargo:rerun-if-changed=wrapper_functions.h");
    println!("cargo:rerun-if-changed=scan_record.cc");
    println!("cargo:rerun-if-changed=scan_record.h");
    println!("cargo:rerun-if-changed=cxx.rs");

    // Link against C++ standard library
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
    
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=msvcrt");
}
