/* Undefined behavior.
Usually it prints an unpredictable integer number.
*/
#include <iostream>
#include <vector>
int main() {
    // A vector is allocated and initialized
    std::vector<int> v { 12 };
    const int& ref_to_first = v[0]; // A reference to it is taken
    v.push_back(13); // The vector is mutated
    std::cout << ref_to_first; // The reference accesses the vector
    // The vector is implicitly deallocated
}
