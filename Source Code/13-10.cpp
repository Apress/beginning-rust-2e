/* It prints:
45, 12, 10, 8, 7, 4, 1, 0, */
#include <array>
#include <algorithm>
#include <iostream>
#include <iterator>
using namespace std;
int main() {
    auto arr = array<int, 8> {
        4, 8, 1, 10, 0, 45, 12, 7 };
    stable_sort(
        arr.begin(), arr.end(),
        [](int a, int b) { return b < a; });
    copy(
        arr.begin(),
        arr.end(),
        ostream_iterator<int>(cout, ", "));
}
