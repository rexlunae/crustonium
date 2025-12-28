// Legacy C++ Component
// This simulates existing Chromium C++ code that needs to be built with Cargo

#include "legacy_component.h"
#include <string>
#include <vector>

namespace chromium {
namespace prototype {

LegacyComponent::LegacyComponent() : data_() {}

LegacyComponent::~LegacyComponent() = default;

bool LegacyComponent::ProcessData(const std::vector<uint8_t>& input,
                                  std::vector<uint8_t>* output) {
  if (input.empty()) {
    return false;
  }

  output->clear();
  output->reserve(input.size());

  // Simple processing: increment each byte
  for (uint8_t byte : input) {
    output->push_back(byte + 1);
  }

  return true;
}

std::string LegacyComponent::GetVersion() const {
  return "1.0.0-prototype";
}

size_t LegacyComponent::GetDataSize() const {
  return data_.size();
}

}  // namespace prototype
}  // namespace chromium
