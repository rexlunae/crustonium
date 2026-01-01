// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Build script for User Data Importer Parsing FFI

use std::path::Path;

fn main() {
    // Get the workspace root directory by going up from the manifest directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    // Use Path to handle both Unix and Windows path separators
    let manifest_path = Path::new(&manifest_dir);
    
    // Navigate up: parsing_ffi -> utility -> user_data_importer -> components -> workspace_root
    let workspace_root = manifest_path
        .parent()  // utility/
        .and_then(|p| p.parent())  // user_data_importer/
        .and_then(|p| p.parent())  // components/
        .and_then(|p| p.parent())  // workspace root
        .expect("Failed to find workspace root");

    // Build the cxx bridge
    cxx_build::bridge("lib.rs")
        .flag_if_supported("-std=c++17")
        .include(workspace_root)  // Add the workspace root to include paths
        .compile("parsing_ffi");

    println!("cargo:rerun-if-changed=lib.rs");
    println!("cargo:rerun-if-changed=history.rs");
    println!("cargo:rerun-if-changed=json.rs");
    println!("cargo:rerun-if-changed=models.rs");
    println!("cargo:rerun-if-changed=utils.rs");
    println!("cargo:rerun-if-changed=zip_archive.rs");
}
