// cxx bridge implementation for C++ side
#include "cargo-cpp-integration-prototype/src/ffi.rs.h"
#include "bridge_wrapper.h"
#include "legacy_component.h"
#include <memory>

namespace chromium {
namespace prototype {
namespace bridge {

// Implementation of functions called from Rust
std::unique_ptr<chromium::prototype::LegacyComponent> create_legacy_component() {
  return std::make_unique<chromium::prototype::LegacyComponent>();
}

bool process_via_cpp(const chromium::prototype::LegacyComponent& component,
                     rust::Slice<const uint8_t> input,
                     rust::Vec<uint8_t>& output) {
  std::vector<uint8_t> cpp_input(input.begin(), input.end());
  std::vector<uint8_t> cpp_output;
  
  // Call the legacy C++ method (const_cast for prototype only)
  bool success = const_cast<chromium::prototype::LegacyComponent&>(component)
                    .ProcessData(cpp_input, &cpp_output);
  
  if (success) {
    output.clear();
    for (uint8_t byte : cpp_output) {
      output.push_back(byte);
    }
  }
  
  return success;
}

rust::String get_cpp_version(const chromium::prototype::LegacyComponent& component) {
  std::string version = component.GetVersion();
  return rust::String(version);
}

}  // namespace bridge
}  // namespace prototype
}  // namespace chromium
