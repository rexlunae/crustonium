// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Build script for Rust GTest Interop
// This component doesn't use cxx, so the build script is minimal

fn main() {
    println!("cargo:rerun-if-changed=rust_gtest_interop.rs");
    println!("cargo:rerun-if-changed=gtest_attribute.rs");
    println!("cargo:rerun-if-changed=expect_macros.rs");
}
