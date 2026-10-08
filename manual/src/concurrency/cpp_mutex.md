# Spot The Difference - C++ Mutex

It's pretty easy to use a Mutex in C++ to never have a data race again:

```cpp
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
```

Except...

```
./mutex_spot 
19728
./mutex_spot 
19453
./mutex_spot 
19517
```

Look closely. I forgot to lock the mutex in `t2`, even though I captured it! That's a *really common bug*.