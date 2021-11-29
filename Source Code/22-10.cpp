/* It prints:
0 4 4*/
#include <iostream>
#include <string>
int main() {
    auto s1 = std::string { "abcd" };
    const auto s2 = s1;
    const auto s3 = move(s1);
    std::cout << s1.size() << " "
        << s2.size() << " " << s3.size();
}
