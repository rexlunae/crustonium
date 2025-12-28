// C++ Utilities
#include "utilities.h"
#include <algorithm>

namespace chromium {
namespace prototype {
namespace utils {

std::vector<uint8_t> ReverseBytes(const std::vector<uint8_t>& input) {
  std::vector<uint8_t> result = input;
  std::reverse(result.begin(), result.end());
  return result;
}

size_t CountNonZero(const std::vector<uint8_t>& input) {
  return std::count_if(input.begin(), input.end(),
                       [](uint8_t byte) { return byte != 0; });
}

}  // namespace utils
}  // namespace prototype
}  // namespace chromium
