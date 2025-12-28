// Legacy C++ Component Header
#ifndef PROTOTYPES_CARGO_CPP_INTEGRATION_LEGACY_COMPONENT_H_
#define PROTOTYPES_CARGO_CPP_INTEGRATION_LEGACY_COMPONENT_H_

#include <cstdint>
#include <string>
#include <vector>

namespace chromium {
namespace prototype {

class LegacyComponent {
 public:
  LegacyComponent();
  ~LegacyComponent();

  // Not copyable or movable
  LegacyComponent(const LegacyComponent&) = delete;
  LegacyComponent& operator=(const LegacyComponent&) = delete;

  bool ProcessData(const std::vector<uint8_t>& input,
                   std::vector<uint8_t>* output);

  std::string GetVersion() const;
  size_t GetDataSize() const;

 private:
  std::vector<uint8_t> data_;
};

}  // namespace prototype
}  // namespace chromium

#endif  // PROTOTYPES_CARGO_CPP_INTEGRATION_LEGACY_COMPONENT_H_
