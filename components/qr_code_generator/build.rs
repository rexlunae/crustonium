// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Build script for QR code generator FFI glue

use std::path::Path;

fn main() {
    // Get the workspace root directory by going up from the manifest directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    // Use Path to handle both Unix and Windows path separators
    let manifest_path = Path::new(&manifest_dir);
    
    // Navigate up: qr_code_generator -> components -> workspace_root
    let workspace_root = manifest_path
        .parent()  // components/
        .and_then(|p| p.parent())  // workspace root
        .expect("Failed to find workspace root");

    // Build the cxx bridge
    cxx_build::bridge("qr_code_generator_ffi_glue.rs")
        .flag_if_supported("-std=c++17")
        .include(workspace_root)  // Add the workspace root to include paths
        .compile("qr_code_generator_ffi_glue");

    println!("cargo:rerun-if-changed=qr_code_generator_ffi_glue.rs");
    println!("cargo:rerun-if-changed=error.h");
}
