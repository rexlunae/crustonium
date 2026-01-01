// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Compatibility shim for chromium::import! macro in Cargo builds
// In GN builds, this macro handles importing dependencies
// In Cargo builds, dependencies are handled via Cargo.toml

/// Stub macro to handle chromium::import! in Cargo builds
/// The actual dependency is handled via Cargo.toml
#[macro_export]
macro_rules! import {
    ($($path:literal);* $(;)?) => {
        // No-op in Cargo builds - dependencies are handled by Cargo.toml
    };
}
