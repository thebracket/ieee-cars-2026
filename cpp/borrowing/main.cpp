#include <string>
#include <iostream>
#include <memory>

struct Test {
    int n;
    Test(int n) : n(n) {} ;
};

int main() {
    auto tmp = std::make_unique<Test>(1);
    auto n_ptr = &tmp->n;

    // All good
    std::cout << tmp->n << '\n';
    std::cout << *n_ptr << '\n';

    tmp.reset();

    // Oh dear
    std::cout << tmp->n << '\n';
    std::cout << *n_ptr << '\n';
}
