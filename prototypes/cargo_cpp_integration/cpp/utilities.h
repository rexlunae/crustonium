// C++ Utilities Header
#ifndef PROTOTYPES_CARGO_CPP_INTEGRATION_UTILITIES_H_
#define PROTOTYPES_CARGO_CPP_INTEGRATION_UTILITIES_H_

#include <cstdint>
#include <vector>

namespace chromium {
namespace prototype {
namespace utils {

std::vector<uint8_t> ReverseBytes(const std::vector<uint8_t>& input);
size_t CountNonZero(const std::vector<uint8_t>& input);

}  // namespace utils
}  // namespace prototype
}  // namespace chromium

#endif  // PROTOTYPES_CARGO_CPP_INTEGRATION_UTILITIES_H_
