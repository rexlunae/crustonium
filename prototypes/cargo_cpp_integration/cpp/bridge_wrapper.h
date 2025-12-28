// Bridge wrapper header
#ifndef PROTOTYPES_CARGO_CPP_INTEGRATION_BRIDGE_WRAPPER_H_
#define PROTOTYPES_CARGO_CPP_INTEGRATION_BRIDGE_WRAPPER_H_

#include <memory>
#include "legacy_component.h"
#include "rust/cxx.h"

namespace chromium {
namespace prototype {
namespace bridge {

// Bridge functions that wrap the LegacyComponent
std::unique_ptr<chromium::prototype::LegacyComponent> create_legacy_component();

bool process_via_cpp(const chromium::prototype::LegacyComponent& component,
                     rust::Slice<const uint8_t> input,
                     rust::Vec<uint8_t>& output);

rust::String get_cpp_version(const chromium::prototype::LegacyComponent& component);

}  // namespace bridge
}  // namespace prototype
}  // namespace chromium

#endif  // PROTOTYPES_CARGO_CPP_INTEGRATION_BRIDGE_WRAPPER_H_
