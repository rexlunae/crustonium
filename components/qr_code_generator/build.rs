// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Build script for QR code generator FFI glue

fn main() {
    // Get the workspace root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    let workspace_root = manifest_dir
        .rsplit_once("/components/qr_code_generator")
        .expect("Expected to be in components/qr_code_generator")
        .0;

    // Build the cxx bridge
    cxx_build::bridge("qr_code_generator_ffi_glue.rs")
        .flag_if_supported("-std=c++17")
        .include(workspace_root)  // Add the workspace root to include paths
        .compile("qr_code_generator_ffi_glue");

    println!("cargo:rerun-if-changed=qr_code_generator_ffi_glue.rs");
    println!("cargo:rerun-if-changed=error.h");
}
