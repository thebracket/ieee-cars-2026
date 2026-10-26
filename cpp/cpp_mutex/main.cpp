#include <iostream>
#include <thread>
#include <mutex>

int main() {
  std::mutex lock;
  int my_safe_data = 0;

  auto t1 = std::thread([&lock, &my_safe_data] () {
    for (int i=0; i<10000; i++) {
      std::lock_guard guard(lock);
      my_safe_data++;
    }
  });
  auto t2 = std::thread([&lock, &my_safe_data] () {
    for (int i=0; i<10000; i++) {
      my_safe_data++;
    }
  });

  t1.join();
  t2.join();

  std::cout << my_safe_data << "\n";
  return 0;
}
