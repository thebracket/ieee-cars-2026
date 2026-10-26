#include <thread>
#include <vector>
#include <iostream>
#include <atomic>

int main() {
  std::atomic_int counter = 0;
  std::vector<std::thread> handles;

  for (int i=0; i<3; i++) {
    handles.push_back(
      std::thread([&counter]() {
        for (int i=0; i<100000; i++) {
          counter++;
        }
      })
    );
  }

  for (auto& handle : handles) {
    handle.join();
  }

  std::cout << "Counter: " << counter << "\n";
  return 0;
}
