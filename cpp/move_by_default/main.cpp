#include <iostream>
#include <string>
#include <utility>

void printme(std::string s) {
  std::cout << s << '\n';
}

int main() {
  std::string s = "hello world";
  printme(std::move(s));
  // Moved-from: valid but unspecified. Usually reset to empty,
  // but don't rely on that; printing the valid string is permitted.
  std::cout << s << '\n';
}
