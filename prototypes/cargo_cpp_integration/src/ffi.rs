// FFI Bridge using cxx
//
// This module defines the FFI interface between Rust and C++ using the cxx crate

#[cxx::bridge]
pub mod ffi {
    // C++ types and functions
    unsafe extern "C++" {
        include!("legacy_component.h");
        include!("bridge_wrapper.h");

        // Opaque C++ type from chromium::prototype namespace
        #[namespace = "chromium::prototype"]
        type LegacyComponent;

        // Bridge functions in chromium::prototype::bridge namespace
        #[namespace = "chromium::prototype::bridge"]
        fn create_legacy_component() -> UniquePtr<LegacyComponent>;
        
        #[namespace = "chromium::prototype::bridge"]
        fn process_via_cpp(
            component: &LegacyComponent,
            input: &[u8],
            output: &mut Vec<u8>,
        ) -> bool;
        
        #[namespace = "chromium::prototype::bridge"]
        fn get_cpp_version(component: &LegacyComponent) -> String;
    }
}

// Re-export for convenience
pub use ffi::*;
