/* It prints:
0 1 1 12345 12345*/
#include <iostream>
#include <memory>
int main() {
    auto i1 = std::unique_ptr<short> {
        new short(12345)
    };
    const auto i2 = std::unique_ptr<short> {
        new short(*i1)
    };
    const auto i3 = move(i1);
    std::cout << (bool)i1 << " " << (bool)i2 << " "
        << (bool)i3 << " " << *i2 << " " << *i3;
}
