/* Undefined behavior.
Usually it prints an unpredictable integer number.
*/
#include <stdio.h>
#include <stdlib.h>
int main() {
    // A vector is allocated and initialized
    int* v = malloc(1 * sizeof (int));
    v[0] = 12;
    // A reference to it is taken
    const int* ref_to_first = &v[0];
    // The vector is mutated
    v = realloc(v, 2 * sizeof (int));
    v[1] = 13;
    // The reference accesses the vector
    printf("%d", *ref_to_first);
    // The vector is explicitly deallocated
    free(v);
}
