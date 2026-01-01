//! Cargo + C++ Integration Prototype
//!
//! Phase 1.1: Research and Prototyping
//!
//! This prototype demonstrates:
//! 1. Building C++ code via the `cc` crate in build.rs
//! 2. Using cxx for type-safe FFI between Rust and C++
//! 3. Performance comparison between Rust and C++ implementations

pub mod ffi;

/// Rust implementation of the same functionality for comparison
pub struct RustComponent {
    data: Vec<u8>,
}

impl RustComponent {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn process_data(&mut self, input: &[u8]) -> Result<Vec<u8>, String> {
        if input.is_empty() {
            return Err("Input cannot be empty".to_string());
        }

        // Same logic as C++: increment each byte
        let output: Vec<u8> = input.iter().map(|&byte| byte.wrapping_add(1)).collect();
        
        Ok(output)
    }

    pub fn get_version(&self) -> &str {
        "1.0.0-prototype-rust"
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }
}

impl Default for RustComponent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_component_basic() {
        let mut component = RustComponent::new();
        let input = vec![0, 1, 2, 255];
        let output = component.process_data(&input).unwrap();
        
        assert_eq!(output, vec![1, 2, 3, 0]); // 255 + 1 wraps to 0
    }

    #[test]
    fn test_rust_component_empty_input() {
        let mut component = RustComponent::new();
        let result = component.process_data(&[]);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_cpp_component_basic() {
        let component = ffi::create_legacy_component();
        let input: Vec<u8> = vec![0, 1, 2, 255];
        let mut output = Vec::new();
        
        let success = ffi::process_via_cpp(&component, &input, &mut output);
        
        assert!(success);
        assert_eq!(output, vec![1, 2, 3, 0]);
    }

    #[test]
    fn test_cpp_component_empty_input() {
        let component = ffi::create_legacy_component();
        let input: Vec<u8> = vec![];
        let mut output = Vec::new();
        
        let success = ffi::process_via_cpp(&component, &input, &mut output);
        
        assert!(!success);
    }

    #[test]
    fn test_cpp_version() {
        let component = ffi::create_legacy_component();
        let version = ffi::get_cpp_version(&component);
        
        assert_eq!(version, "1.0.0-prototype");
    }
}
