/* ILLEGAL. The C++ compiler prints an error message like:
no matching function for call to ’sqrt’
or like:
cannot convert ‘const char*’ to ‘double’
*/
#include <iostream>
#include <cmath>
template <typename Number>
Number quartic_root(Number x) {
    return sqrt(sqrt(x));
}
int main() {
    std::cout << quartic_root("Hello");
}
