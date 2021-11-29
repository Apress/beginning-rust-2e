/* It prints:
60, 10000000, 183.19, Q*/
#include <stdio.h>
int main() {
    struct SomeData {
        int integer;
        float fractional;
        char character;
        unsigned char five_bytes[5];
    };
    struct SomeData data = {
        10000000,
        183.19,
        'Q',
        {9, 0, 250, 60, 200},
    };
    printf("%d, %d, %g, %c",
        data.five_bytes[3], data.integer,
        data.fractional, data.character);
    return 0;
}
