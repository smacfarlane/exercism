#include <string>

namespace log_line {
  std::string message(std::string input) {
    int pos = input.find(":") + 2;
    int len = input.length();

    // Should handle cases where it isn't found or there is no message;

    return input.substr(pos, len);
  }

  std::string log_level(std::string input) {
    int left = input.find("[") + 1;
    int right = input.find("]") - left;
    return input.substr(left, right);
  }

  std::string reformat(std::string input) { 
    return message(input) + " (" + log_level(input) + ")";
  }
}