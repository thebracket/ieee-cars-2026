# C++ Atomic

> In fact, when Go gained the abilty to track data races, Uber found *thousands* of them that could affect prices and journeys! Mozilla had such problems with it that it was the inspiration to start making Rust.

Most C++ programmers will tell me that (a) parallelizing a simple sum is pretty pointless, and (b) you can fix the issue with an atomic.

```cpp
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

  for (int i=0; i<handles.size(); i++) {
    handles[i].join();
  }

  std::cout << "Counter: " << counter << "\n";
  return 0;
}
```

And now you get the correct answer every time. That's great! Wouldn't it have been nice if the first version gave you a compilation error?